mod imp;
use std::fs::File;
use gtk::{glib,gio,NoSelection,CustomFilter,FilterListModel,CheckButton,Align,
    pango,ListBoxRow,Label, Entry, Button,Box};
use glib::{Object,clone};
use gio::Settings;
use adw::{prelude::*,subclass::prelude::*,ActionRow};
use crate::task_object::TaskObject;
use crate::APP_ID;
use crate::utils::data_path;
use crate::collection_object::{CollectionObject, CollectionData};
const APP_ID_C:&str="org.gtk_rs.TodoNewCollection";
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
    fn tasks(&self)->gio::ListStore {
        self.current_collection().tasks()
    }
    fn current_collection(&self)->CollectionObject {
        self.imp()
            .current_collection
            .borrow()
            .clone()
            .expect("`current_collection` should be set in `set_current_collections`")
    }
    fn collections(&self)->gio::ListStore {
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
            .expect("`current_filter_model` should be set in `set_current_collection`")
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
        let collections=gio::ListStore::new::<CollectionObject>();
        self.imp()
            .collections
            .set(collections.clone())
            .expect("could not set collections");

        self.imp().collections_list.bind_model(
            Some(&collections), 
            clone!(@weak self as window=>@default-panic,move|obj|{
                let colletion_object=obj
                    .downcast_ref()
                    .expect("The object should be of type 'CollectionObject'");
                let row=window.create_collection_row(colletion_object);
                row.upcast()
            })
        )
    }
    fn restore_data(&self) {
        if let Ok(file) = File::open(data_path()) {
            let backup_data:Vec<CollectionData>=serde_json::from_reader(file)
                .expect("deberia leer el 'backup_data' del archivo json");

            let collections:Vec<CollectionObject>=backup_data
                .into_iter()
                .map(CollectionObject::from_collection_data)
                .collect();
            self.collections().extend_from_slice(&collections);

            if let Some(first_collection) = collections.first() {
                self.set_current_collection(first_collection.clone());
            }
        }
    }
    fn create_collection_row(&self,collection_object:&CollectionObject)->ListBoxRow {
        let label=Label::builder()
            .ellipsize(pango::EllipsizeMode::End)
            .xalign(0.0)
            .build();

        collection_object
            .bind_property("title", &label, "label")
            .sync_create()
            .build();
        ListBoxRow::builder().child(&label).build()
    }
    fn set_current_collection(&self,collection:CollectionObject) {
        let tasks=collection.tasks();
        let filter_model=FilterListModel::new(Some(tasks.clone()), self.filter());
        let selection_model=NoSelection::new(Some(filter_model.clone()));
        self.imp().tasks_list.bind_model(
            Some(&selection_model), 
            clone!(@weak self as window=>@default-panic,move|obj|{
                let task_object=obj
                    .downcast_ref()
                    .expect("The object should be of type 'TaskObject'");
                let row=window.create_task_row(task_object);
                row.upcast()
            })
        );
        self.imp().current_filter_model.replace(Some(filter_model));

        if let Some(handler_id) = self.imp().tasks_changed_handler_id.take() {
            self.tasks().disconnect(handler_id);
        }
        
        self.set_task_list_visible(&tasks);
        let tasks_changed_handler_id=tasks.connect_items_changed(
            clone!(@weak self as window=>move|tasks,_,_,_|{
                window.set_task_list_visible(tasks);
            })
        );
        self.imp()
            .tasks_changed_handler_id
            .replace(Some(tasks_changed_handler_id));

        self.imp().current_collection.replace(Some(collection));
        self.select_collection_row();
    }
    fn set_task_list_visible(&self,tasks:&gio::ListStore) {
        self.imp().tasks_list.set_visible(tasks.n_items() > 0);
    }
    fn select_collection_row(&self) {
        if let Some(index) = self.collections().find(&self.current_collection()) {
            let row=self.imp().collections_list.row_at_index(index as i32);
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
            // .css_name("rows")
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
                window.new_task();
            }));
        self.settings().connect_changed(
            Some("filter"), 
            clone!(@weak self as window=>move|_,_|{
                window.set_filter();
            })
        );
        self.set_stack();
        self.collections().connect_items_changed(clone!(@weak self as window=>move|_,_,_,_|{
            window.set_stack();
        }));
        self.imp().collections_list.connect_row_activated(
            clone!(@weak self as window=>move|_,row|{
                let index=row.index();
                let selected_collection=window.collections()
                    .item(index as u32)
                    .expect("There needs to be an object at this position")
                    .downcast::<CollectionObject>()
                    .expect("The object has to be CollectionObject");
                window.set_current_collection(selected_collection);
                window.imp().leaflet.navigate(adw::NavigationDirection::Forward);
            })
        );
        self.imp().leaflet.connect_folded_notify(clone!(@weak self as window=>move|leaflet|{
            if leaflet.is_folded() {
                window.imp().collections_list.set_selection_mode(gtk::SelectionMode::None)
            }else {
                window.imp().collections_list.set_selection_mode(gtk::SelectionMode::Single);
                window.select_collection_row();
            }
        }));
        self.imp().back_button.connect_clicked(clone!(@weak self as window=>move|_|{
            window.imp().leaflet.navigate(adw::NavigationDirection::Back);
        }));
    }
    fn set_stack(&self) {
        if self.collections().n_items()>0 {
            self.imp().stack.set_visible_child_name("main");
        }else {
            self.imp().stack.set_visible_child_name("placeholder");
        }
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
        }));
        self.add_action(&action_remove_done_tasks);

        let action_new_list=gio::SimpleAction::new("new-collection", None);
        action_new_list.connect_activate(clone!(@weak self as window=>move|_,_|{
            window.new_collection();
        }));
        self.add_action(&action_new_list);

    }
    fn new_collection(&self) {
        let app=adw::Application::builder().application_id(APP_ID_C).build();
        app.connect_activate(move|app|{
            let entry=Entry::builder()
                .placeholder_text("Name")
                .build();
            let button_create=Button::builder()
                .label("Create")
                .sensitive(true)
                // .action_name("create")
                .build();
            
            let gtk_box_1=Box::builder()
                .orientation(gtk::Orientation::Vertical)
                .margin_top(12)
                .margin_bottom(12)
                .margin_start(12)
                .margin_end(12)
                .spacing(12)
                .build();
            gtk_box_1.append(&entry);
            gtk_box_1.append(&button_create);
            let window=gtk::ApplicationWindow::builder()
                .application(app)
                .title("New Collection")
                .child(&gtk_box_1)
                .build();
            button_create.connect_clicked(clone!(@weak entry=>move|_|{
                let texto=entry.buffer().text().to_string();
                println!("{:?}",texto);
                entry.buffer().set_text("");
            }));

            window.present();
        });
        app.run();
    }
}
