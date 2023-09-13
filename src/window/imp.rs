use std::cell::RefCell;
use gtk::{gio,glib,subclass::prelude::*,CompositeTemplate,Entry,ListView};
use glib::subclass::InitializingObject;

#[derive(Default,CompositeTemplate)]
#[template(resource="/org/gtk_rs/Todo/window.ui")]
pub struct Window{
    #[template_child]
    pub entry:TemplateChild<Entry>,
    #[template_child]
    pub tasks_list:TemplateChild<ListView>,
    pub tasks:RefCell<Option<gio::ListStore>>,
}
#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "TodoWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(_klass: &mut Self::Class) {
        _klass.bind_template();
    }
    fn instance_init(_obj: &InitializingObject<Self>) {
        _obj.init_template();
    }
}
impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();
        let obj=self.obj();
        obj.setup_callbacks();
        obj.setup_factory();
        obj.setup_tasks();
    }
}
impl WindowImpl for Window {}
impl ApplicationWindowImpl for Window {}
impl WidgetImpl for Window {}