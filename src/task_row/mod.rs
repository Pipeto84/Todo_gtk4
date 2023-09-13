mod imp;

use gtk::{glib,subclass::prelude::*, prelude::*,pango};
use glib::Object;
use pango::{AttrInt,AttrList};

use crate::task_object::TaskObject;

glib::wrapper! {
    pub struct TaskRow(ObjectSubclass<imp::TaskRow>)
        @extends gtk::Box,gtk::Widget,
        @implements gtk::Accessible,gtk::Buildable,gtk::ConstraintTarget,gtk::Orientable;
}
impl TaskRow {
    pub fn new()->Self {
        Object::builder().build()
    }
    pub fn bind(&self,task_object:&TaskObject) {
        let completed_button=self.imp().completed_button.get();
        let content_label=self.imp().content_label.get();
        let mut bindings=self.imp().bindings.borrow_mut();

        let completed_button_binding=task_object
            .bind_property("completed", &completed_button, "active")
            .bidirectional()
            .sync_create()
            .build();
        bindings.push(completed_button_binding);

        let content_label_binding=task_object
            .bind_property("content", &content_label, "label")
            .sync_create()
            .build();
        bindings.push(content_label_binding);

        let content_label_binding=task_object
            .bind_property("completed", &content_label, "attributes")
            .sync_create()
            .transform_to(|_x,active|{
                let attribute_list=AttrList::new();
                if active {
                    let attribute=AttrInt::new_strikethrough(true);
                    attribute_list.insert(attribute);
                }
                Some(attribute_list.to_value())
            }).build();
        bindings.push(content_label_binding);
    }
    pub fn unbind(&self) {
        for binding in self.imp().bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }
}
impl Default for TaskRow {
    fn default() -> Self {
        Self::new()
    }
}