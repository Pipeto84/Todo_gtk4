use std::cell::RefCell;
use gtk::{glib,subclass::prelude::*,CompositeTemplate,CheckButton,Label,};
use glib::Binding;

#[derive(Default,CompositeTemplate)]
#[template(resource="/org/gtk_rs/Todo/task_row.ui")]
pub struct TaskRow{
    #[template_child]
    pub completed_button:TemplateChild<CheckButton>,
    #[template_child]
    pub content_label:TemplateChild<Label>,
    pub bindings:RefCell<Vec<Binding>>,
}
#[glib::object_subclass]
impl ObjectSubclass for TaskRow {
    const NAME: &'static str = "TodoTaskRow";
    type Type = super::TaskRow;
    type ParentType = gtk::Box;

    fn class_init(_klass: &mut Self::Class) {
        _klass.bind_template();
        _klass.set_css_name("task-row");
    }
    fn instance_init(_obj: &glib::subclass::InitializingObject<Self>) {
        _obj.init_template();
    }
}
impl ObjectImpl for TaskRow {}
impl WidgetImpl for TaskRow {}
impl BoxImpl for TaskRow {}