use std::cell::RefCell;

use glib::Properties;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use super::ChatData;

// Object holding the state
#[derive(Properties, Default)]
#[properties(wrapper_type = super::ChatObject)]
pub struct ChatObject {
    #[property(name = "incoming", get, set, type = bool, member = incoming)]
    #[property(name = "content", get, set, type = String, member = content)]
    pub data: RefCell<ChatData>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for ChatObject {
    const NAME: &'static str = "ChatObject";
    type Type = super::ChatObject;
}

// Trait shared by all GObjects
#[glib::derived_properties]
impl ObjectImpl for ChatObject {}