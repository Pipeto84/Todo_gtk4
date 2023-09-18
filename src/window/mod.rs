mod imp;
use std::fs::File;
use gtk::{glib,gio,NoSelection,CustomFilter,FilterListModel,CheckButton,Align};
use glib::{Object,clone};
use gio::Settings;
use adw::{prelude::*,subclass::prelude::*,ActionRow};

use crate::{task_object::{TaskObject, TaskData},APP_ID,utils::data_path};

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow,gtk::Window,gtk::Widget,
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
        self.imp().tasks_list.bind_model(
            Some(&selection_model), 
            clone!(@weak self as window=>@default-panic,move|obj|{
                let task_object=obj.downcast_ref().expect("el objeto deberia ser TaskObject");
                let row=window.create_task_row(task_object);
                row.upcast()
            })
        );
        self.settings().connect_changed(Some("filter"), 
            clone!(@weak self as window,@weak filter_model =>move|_,_|{
                filter_model.set_filter(window.filter().as_ref());
            })
        );
        self.set_task_list_visible(&self.tasks());
        self.tasks().connect_items_changed(
            clone!(@weak self as window=>move|tasks,_x,_y,_z|{
                window.set_task_list_visible(tasks);
                println!("{:?}-{:?}-{:?}",_x,_y,_z);
            })
        );
    }
    fn set_task_list_visible(&self,tasks:&gio::ListStore) {
        self.imp().tasks_list.set_visible(tasks.n_items() > 0);
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
    fn create_task_row(&self,task_object:&TaskObject)->ActionRow {
        let checkbutton=CheckButton::builder()
            .valign(Align::Center)
            .can_focus(false)
            .build();
        let row=ActionRow::builder()
            .activatable_widget(&checkbutton)
            .css_name("rows")
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)   
            .height_request(50)         
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
}
