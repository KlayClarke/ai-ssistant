mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
    pub struct ChatObject(ObjectSubclass<imp::ChatObject>);
}

impl ChatObject {
    pub fn new(incoming: bool, content: String) -> Self {
        Object::builder()
            .property("incoming", incoming)
            .property("content", content)
            .build()
    }
}

#[derive(Default)]
pub struct ChatData {
    pub incoming: bool,
    pub content: String,
}