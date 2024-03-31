use std::cell::RefCell;

use glib::subclass::InitializingObject;
use gtk::ListView;
use gtk::subclass::prelude::*;
use gtk::{glib, Entry, CompositeTemplate};
use gio::ListStore;

use crate::chat_object::{ChatData, ChatObject};

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/example/window.xml")]
pub struct Window {
    #[template_child]
    pub entry: TemplateChild<Entry>,
    #[template_child]
    pub chat_view: TemplateChild<ListView>,
    pub current_chat: RefCell<Option<ChatObject>>,
    pub chats: RefCell<Option<ListStore>>,
    pub chats_vec: RefCell<Option<Vec<ChatData>>>
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "MyGtkAppWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for Window {
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();

        let obj = self.obj();
        obj.setup_chats();
        obj.setup_callbacks();
        obj.setup_factory();
    }
}

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}