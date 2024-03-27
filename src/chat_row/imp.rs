use std::cell::RefCell;

use glib::Binding;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, Label};

// Object holding the state
#[derive(Default, CompositeTemplate)]
#[template(resource = "/org/gtk_rs/example/chat_row.xml")]
pub struct ChatRow {
    #[template_child]
    pub content_label: TemplateChild<Label>,
    // Vector holding the bindings to properties of `TaskObject`
    pub bindings: RefCell<Vec<Binding>>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for ChatRow {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "ChatRow";
    type Type = super::ChatRow;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.set_css_name("chat-row")
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }

}

// Trait shared by all GObjects
impl ObjectImpl for ChatRow {}

// Trait shared by all widgets
impl WidgetImpl for ChatRow {}

// Trait shared by all boxes
impl BoxImpl for ChatRow {}