mod imp;
use std::fs::File;
use gtk::{glib,gio,prelude::*,subclass::prelude::*,Application,NoSelection, SignalListItemFactory, ListItem, 
    CustomFilter, FilterListModel};
use glib::{Object,clone};
use gio::Settings;

use crate::{task_object::{TaskObject, TaskData}, task_row::TaskRow, APP_ID, utils::data_path};

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow,gtk::Window,gtk::Widget,
        @implements gio::ActionGroup,gio::ActionMap,gtk::Accessible,gtk::Buildable,
                    gtk::ConstraintTarget,gtk::Native,gtk::Root,gtk::ShortcutManager;
}
impl Window {
    pub fn new(app:&Application)->Self {
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
        self.imp()
            .tasks
            .borrow()
            .clone()
            .expect("no pudo obtener las tareas actuales")
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
    fn setup_tasks(&self) {
        let model=gio::ListStore::new::<TaskObject>();
        self.imp().tasks.replace(Some(model));

        let filter_model=FilterListModel::new(Some(self.tasks()), self.filter());
        let selection_model=NoSelection::new(Some(filter_model.clone()));
        self.imp().tasks_list.set_model(Some(&selection_model));

        self.settings().connect_changed(Some("filter"), 
            clone!(@weak self as window,@weak filter_model =>move|_,_|{
                filter_model.set_filter(window.filter().as_ref());
            })
        );
    }
    fn restore_data(&self) {
        if let Ok(file) = File::open(data_path()) {
            let backup_data:Vec<TaskData>=serde_json::from_reader(file)
                .expect("deberia leer el 'backup_data' del archivo json");

            let task_object:Vec<TaskObject>=backup_data
                .into_iter()
                .map(TaskObject::from_task_data)
                .collect();
            self.tasks().extend_from_slice(&task_object);
        }
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
                window.new_task();println!("{:?}",_x);
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
    fn setup_factory(&self) {
        let factory=SignalListItemFactory::new();

        factory.connect_setup(move|_,list_item|{
            let task_row=TaskRow::new();
            list_item
                .downcast_ref::<ListItem>()
                .expect("tiene que ser ListItem")
                .set_child(Some(&task_row));
        });
        factory.connect_bind(move|_,list_item|{
            let task_row=list_item
                .downcast_ref::<ListItem>()
                .expect("tiene que ser ListItem")
                .child()
                .and_downcast::<TaskRow>()
                .expect("debe ser TaskRow");
            let task_object=list_item
                .downcast_ref::<ListItem>()
                .expect("tiene que ser ListItem")
                .item()
                .and_downcast::<TaskObject>()
                .expect("debe ser TaskObject");
            task_row.bind(&task_object);
        });
        factory.connect_unbind(move|_,list_item|{
            let task_row=list_item
                .downcast_ref::<ListItem>()
                .expect("tiene que ser ListItem")
                .child()
                .and_downcast::<TaskRow>()
                .expect("debe ser TaskRow");
            task_row.unbind();
        });
        self.imp().tasks_list.set_factory(Some(&factory));
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
}
