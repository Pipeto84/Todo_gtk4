mod imp;
use std::fs::File;
use gtk::{glib,gio,NoSelection,CustomFilter,FilterListModel,CheckButton,Align,
    ListBoxRow,Label,pango};
use glib::{Object,clone};
use gio::Settings;
use adw::{prelude::*,subclass::prelude::*,ActionRow};

use crate::{task_object::TaskObject,APP_ID,utils::data_path};
use crate::collection_object::{CollectionObject,CollectionData};
glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends adw::ApplicationWindow,gtk::ApplicationWindow,gtk::Window,gtk::Widget,
        @implements gio::ActionGroup,gio::ActionMap,gtk::Accessible,gtk::Buildable,
                    gtk::ConstraintTarget,gtk::Native,gtk::Root,gtk::ShortcutManager;
}
impl Window {
    pub fn new(app:&adw::Application)->Self {
        Object::builder().property("application", app).build()
    }
    fn setup_settings(&self) {
        let settings=Settings::new(APP_ID);
        self.imp()
            .settings
            .set(settings)
            .expect("primero setup_settings que settings");
    }
    fn settings(&self)->&Settings {
        self.imp()
            .settings
            .get()
            .expect("no se organizo settings en setup_settings")
    }
    fn tasks(&self) -> gio::ListStore {
        self.current_collection().tasks()
    }
    fn current_collection(&self) -> CollectionObject {
        self.imp()
            .current_collection
            .borrow()
            .clone()
            .expect("`current_collection` should be set in `set_current_collections`.")
    }
    fn collections(&self) -> gio::ListStore {
        self.imp()
            .collections
            .get()
            .expect("`collections` should be set in `setup_collections`.")
            .clone()
    }
    fn set_filter(&self) {
        self.imp()
            .current_filter_model
            .borrow()
            .clone()
            .expect("`current_filter_model` should be set in `set_current_collection`.")
            .set_filter(self.filter().as_ref());
    }
    fn filter(&self)->Option<CustomFilter> {
        let filter_state:String=self.settings().get("filter");

        let filter_open=CustomFilter::new(|obj|{
            let task_object=obj
                .downcast_ref::<TaskObject>()
                .expect("el objeto tiene que ser TaskObject");
            !task_object.is_completed()
        });
        let filter_done=CustomFilter::new(|obj|{
            let task_object=obj
                .downcast_ref::<TaskObject>()
                .expect("el objeto tiene que ser TaskObject");
            task_object.is_completed()
        });
        match filter_state.as_str() {
            "All"=>None,
            "Done"=>Some(filter_done),
            "Open"=>Some(filter_open),
            _ => unreachable!( ),
        }
    }
    fn setup_collections(&self) {
        let collections = gio::ListStore::new::<CollectionObject>();
        self.imp()
            .collections
            .set(collections.clone())
            .expect("Could not set collections");

        self.imp().collections_list.bind_model(
            Some(&collections),
            clone!(@weak self as window => @default-panic, move |obj| {
                let collection_object = obj
                    .downcast_ref()
                    .expect("The object should be of type `CollectionObject`.");
                let row = window.create_collection_row(collection_object);
                row.upcast()
            }),
        )
    }
    fn restore_data(&self) {
        if let Ok(file) = File::open(data_path()) {
            let backup_data: Vec<CollectionData> = serde_json::from_reader(file)
                .expect(
                    "It should be possible to read `backup_data` from the json file.",
                );

            let collections: Vec<CollectionObject> = backup_data
                .into_iter()
                .map(CollectionObject::from_collection_data)
                .collect();

            self.collections().extend_from_slice(&collections);

            if let Some(first_collection) = collections.first() {
                self.set_current_collection(first_collection.clone());
            }
        }
    }
    fn create_collection_row(&self,collection_object: &CollectionObject) -> ListBoxRow {
        let label = Label::builder()
            .ellipsize(pango::EllipsizeMode::End)
            .xalign(0.0)
            .build();

        collection_object
            .bind_property("title", &label, "label")
            .sync_create()
            .build();

        ListBoxRow::builder().child(&label).build()
    }
    fn set_task_list_visible(&self,tasks:&gio::ListStore) {
        self.imp().tasks_list.set_visible(tasks.n_items() > 0);
    }
    fn select_collection_row(&self) {
        if let Some(index) = self.collections().find(&self.current_collection()) {
            let row = self.imp().collections_list.row_at_index(index as i32);
            self.imp().collections_list.select_row(row.as_ref());
        }
    }
    fn create_task_row(&self,task_object:&TaskObject)->ActionRow {
        let checkbutton=CheckButton::builder()
            .valign(Align::Center)
            .can_focus(false)
            .build();
        let row=ActionRow::builder()
            .activatable_widget(&checkbutton)
            .css_name("rows")
            .margin_top(6)
            .margin_bottom(6)
            .margin_start(12)
            .margin_end(12)   
            .height_request(40)         
            .build();
        row.add_prefix(&checkbutton);

        task_object
            .bind_property("completed", &checkbutton, "active")
            .bidirectional()
            .sync_create()
            .build();
        task_object
            .bind_property("content", &row, "title")
            .sync_create()
            .build();

        row
    }
    fn setup_callbacks(&self) {
        self.imp()
            .entry
            .connect_activate(clone!(@weak self as window=>move|_|{
                window.new_task();
            }));
        self.imp()
            .entry
            .connect_icon_release(clone!(@weak self as window=>move|_,_x|{
            }));
    }
    fn new_task(&self) {
        let buffer=self.imp().entry.buffer();
        let content=buffer.text().to_string();
        if content.is_empty() {
            return;
        }
        buffer.set_text("");
        let task=TaskObject::new(false, content);
        self.tasks().append(&task);
    }
    fn setup_actions(&self) {
        let action_filter=self.settings().create_action("filter");
        self.add_action(&action_filter);

        let action_remove_done_tasks=
            gio::SimpleAction::new("remove-done-tasks", None);
        action_remove_done_tasks.connect_activate(clone!(@weak self as window=>move|_,_|{
            let tasks=window.tasks();
            let mut position=0;
            while let Some(item) = tasks.item(position) {
                let task_object=item
                    .downcast_ref::<TaskObject>()
                    .expect("el objeto tiene que ser TaskObject");
                if task_object.is_completed() {
                    tasks.remove(position);
                }else {
                    position += 1;
                }
            }
            })
        );
        self.add_action(&action_remove_done_tasks);
    }
    fn set_current_collection(&self, collection: CollectionObject) {
        let tasks = collection.tasks();
        let filter_model = FilterListModel::new(Some(tasks.clone()), self.filter());
        let selection_model = NoSelection::new(Some(filter_model.clone()));
        self.imp().tasks_list.bind_model(
            Some(&selection_model),
            clone!(@weak self as window => @default-panic, move |obj| {
                let task_object = obj
                    .downcast_ref()
                    .expect("The object should be of type `TaskObject`.");
                let row = window.create_task_row(task_object);
                row.upcast()
            }),
        );

        self.imp().current_filter_model.replace(Some(filter_model));

        if let Some(handler_id) = self.imp().tasks_changed_handler_id.take() {
            self.tasks().disconnect(handler_id);
        }

        self.set_task_list_visible(&tasks);
        let tasks_changed_handler_id = tasks.connect_items_changed(
            clone!(@weak self as window => move |tasks, _, _, _| {
                window.set_task_list_visible(tasks);
            }),
        );
        self.imp()
            .tasks_changed_handler_id
            .replace(Some(tasks_changed_handler_id));

        self.imp().current_collection.replace(Some(collection));

        self.select_collection_row();
    }
}
