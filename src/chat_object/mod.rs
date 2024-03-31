mod imp;

use std::path::PathBuf;
use gio::glib::object::ObjectExt;
use glib::Object;
use gtk::glib;
use serde::{Deserialize, Serialize};

glib::wrapper! {
    pub struct ChatObject(ObjectSubclass<imp::ChatObject>);
}

impl ChatObject {
    pub fn new(role: String, content: String, image: Option<String>) -> Self {
        Object::builder()
            .property("role", role)
            .property("content", content)
            .property("image", image)
            .build()
    }

    pub fn get_image_path(&self) -> Option<PathBuf> {
        self.image().map(PathBuf::from)
    }

    pub fn set_user_content(&mut self, content: String) {
        self.set_property("content", content);
    }
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct ChatData {
    pub role: String,
    pub content: String,
    pub image: Option<String>
}