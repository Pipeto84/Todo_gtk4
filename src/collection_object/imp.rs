use std::cell::{RefCell,OnceCell};
use gtk::{gio,glib};
use glib::Properties;
use adw::{prelude::*,subclass::prelude::*};

#[derive(Default,Properties)]
#[properties(wrapper_type=super::CollectionObject)]
pub struct CollectionObject{
    #[property(get,set)]
    pub title:RefCell<String>,
    #[property(get,set)]
    pub tasks:OnceCell<gio::ListStore>,
}
#[glib::object_subclass]
impl ObjectSubclass for CollectionObject {
    const NAME: &'static str = "TodoCollectionObject";
    type Type = super::CollectionObject;
}
#[glib::derived_properties]
impl ObjectImpl for CollectionObject {}
