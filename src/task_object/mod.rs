mod imp;

use gtk::glib;
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
}
#[derive(Default)]
pub struct TaskData{
    pub completed:bool,
    pub content:String,
}