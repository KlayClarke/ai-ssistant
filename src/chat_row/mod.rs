mod imp;

use glib::Object;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, pango};
use pango::{AttrInt, AttrList};

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
        let incoming_icon = self.imp().incoming_icon.get();
        let content_label = self.imp().content_label.get();
        content_label.set_wrap(true);
        let mut bindings = self.imp().bindings.borrow_mut();

        // Bind `chat_object.incoming` to `chat_object.incoming_icon.active`
        let incoming_button_binding = chat_object
            .bind_property("incoming", &incoming_icon, "active")
            .bidirectional()
            .sync_create()
            .build();
        // Save binding
        bindings.push(incoming_button_binding);

        // Bind `chat_object.content` to `chat_row.content_label.label`
        let content_label_binding = chat_object
            .bind_property("content", &content_label, "label")
            .sync_create()
            .build();
        // Save binding
        bindings.push(content_label_binding);

        // Bind `chat_object.completed` to `chat_row.content_label.attributes`
        let content_label_binding = chat_object
            .bind_property("incoming", &content_label, "attributes")
            .sync_create()
            .transform_to(|_, active| {
                let attribute_list = AttrList::new();
                if active {
                    // If "active" is true, content of the label will be strikethrough
                    let attribute = AttrInt::new_strikethrough(true);
                    attribute_list.insert(attribute);
                }
                Some(attribute_list.to_value())
            })
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