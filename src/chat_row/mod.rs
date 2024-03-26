mod imp;

use glib::Object;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::glib;

use crate::chat_object::ChatObject;

glib::wrapper! {
    pub struct ChatRow(ObjectSubclass<imp::ChatRow>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Default for ChatRow {
    fn default() -> Self {
        Self::new()
    }
}

impl ChatRow {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn bind(&self, chat_object: &ChatObject) {
        // Get state
        let content_label = self.imp().content_label.get();
        content_label.set_wrap(true);
        let mut bindings = self.imp().bindings.borrow_mut();

        // Bind `chat_object.content` to `chat_row.content_label.label`
        let content_label_binding = chat_object
            .bind_property("content", &content_label, "label")
            .sync_create()
            .build();
        // Save binding
        bindings.push(content_label_binding);
    }

    pub fn unbind(&self) {
        // Unbind all stored bindings
        for binding in self.imp().bindings.borrow_mut().drain(..) {
            binding.unbind();
        }
    }
}