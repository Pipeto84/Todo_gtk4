use std::{cell::{RefCell,OnceCell},fs::File};
use gtk::{gio,glib,CompositeTemplate,Entry,ListBox,Stack,Button,FilterListModel};
use glib::{subclass::InitializingObject,SignalHandlerId};
use gio::Settings;
use adw::{prelude::*,subclass::prelude::*,Leaflet};

use crate::{task_object::{TaskData, TaskObject}, utils::data_path};
use crate::collection_object::{CollectionObject,CollectionData};
#[derive(Default,CompositeTemplate)]
#[template(resource="/org/gtk_rs/Todo/window.ui")]
pub struct Window{
    pub settings: OnceCell<Settings>,
    #[template_child]
    pub entry: TemplateChild<Entry>,
    #[template_child]
    pub tasks_list: TemplateChild<ListBox>,
    #[template_child]
    pub collections_list: TemplateChild<ListBox>,
    #[template_child]
    pub leaflet: TemplateChild<Leaflet>,
    #[template_child]
    pub stack: TemplateChild<Stack>,
    #[template_child]
    pub back_button: TemplateChild<Button>,
    pub collections: OnceCell<gio::ListStore>,
    pub current_collection: RefCell<Option<CollectionObject>>,
    pub current_filter_model: RefCell<Option<FilterListModel>>,
    pub tasks_changed_handler_id: RefCell<Option<SignalHandlerId>>,
}
#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "TodoWindow";
    type Type = super::Window;
    type ParentType = adw::ApplicationWindow;

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
        obj.setup_settings();
        obj.setup_collections();
        obj.restore_data();
        obj.setup_callbacks();
        obj.setup_actions();
    }
}
// Trait shared by all windows
impl WindowImpl for Window {
    fn close_request(&self) -> glib::Propagation {
        let backup_data: Vec<CollectionData> = self
            .obj()
            .collections()
            .iter::<CollectionObject>()
            .filter_map(|collection_object| collection_object.ok())
            .map(|collection_object| collection_object.to_collection_data())
            .collect();

        let file = File::create(data_path()).expect("Could not create json file.");
        serde_json::to_writer(file, &backup_data)
            .expect("Could not write data to json file");

        self.parent_close_request()
    }
}
impl ApplicationWindowImpl for Window {}
impl WidgetImpl for Window {}
impl AdwApplicationWindowImpl for Window {}