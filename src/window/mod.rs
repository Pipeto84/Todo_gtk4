mod imp;
use gtk::{glib,gio,prelude::*,subclass::prelude::*,Application,NoSelection, SignalListItemFactory, ListItem};
use glib::{Object,clone};

use crate::{task_object::TaskObject, task_row::TaskRow};

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
    fn tasks(&self)->gio::ListStore {
        self.imp()
            .tasks
            .borrow()
            .clone()
            .expect("no pudo obtener las tareas actuales")
    }
    fn setup_tasks(&self) {
        let model=gio::ListStore::new::<TaskObject>();
        self.imp().tasks.replace(Some(model));

        let selection_model=NoSelection::new(Some(self.tasks()));
        self.imp().tasks_list.set_model(Some(&selection_model));
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
}
