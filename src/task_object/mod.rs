mod imp;

use gtk::{glib,subclass::prelude::*};
use glib::Object;

glib::wrapper! {
    pub struct TaskObject(ObjectSubclass<imp::TaskObject>);
}
impl TaskObject {
    pub fn new(completed:bool,content:String)->Self {
        Object::builder()
            .property("completed", completed)
            .property("content", content)
            .build()
    }
    pub fn is_completed(&self)->bool {
        self.imp().data.borrow().completed
    }
    pub fn task_data(&self)->TaskData {
        self.imp().data.borrow().clone()
    }
    pub fn from_task_data(task_data:TaskData)->Self {
        Self::new(task_data.completed, task_data.content)
    }
}
#[derive(Default,Clone)]
pub struct TaskData{
    pub completed:bool,
    pub content:String,
}