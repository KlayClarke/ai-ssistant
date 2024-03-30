mod imp;

use glib::Object;
use gtk::glib;
use serde::{Deserialize, Serialize};

glib::wrapper! {
    pub struct ChatObject(ObjectSubclass<imp::ChatObject>);
}

impl ChatObject {
    pub fn new(role: String, content: String) -> Self {
        Object::builder()
            .property("role", role)
            .property("content", content)
            .build()
    }
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct ChatData {
    pub role: String,
    pub content: String,
}