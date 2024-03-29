mod imp;

use gtk::subclass::prelude::*;
use glib::{Object, clone};
use gtk::{gio, glib, ListItem, SignalListItemFactory, prelude::*, NoSelection, Application};
use reqwest::{Error, Response};
use std::sync::{OnceLock, Arc, Mutex};
use tokio::runtime::Runtime;
use async_channel::Receiver;

use crate::api_types::APIResponse;
use crate::chat_object::ChatObject;
use crate::chat_row::ChatRow;
use crate::api_client::APIClient;

fn runtime() -> &'static Runtime {
    static RUNTIME: OnceLock<Runtime> = OnceLock::new();
    RUNTIME.get_or_init(|| {
        Runtime::new().expect("Setting up tokio runtime needs to succeed.")
    })
}

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        // Create new window
        Object::builder().property("application", app).build()
    }

    fn chats(&self) -> gio::ListStore {
        self.imp()
            .chats
            .borrow()
            .clone()
            .expect("Could not get current chats.")
    }

    fn setup_chats(&self) {
        // Create new model
        let model = gio::ListStore::new::<ChatObject>();

        // Get state and set model
        self.imp().chats.replace(Some(model));

        // Wrap model with selection and pass it to the list view
        let selection_model = NoSelection::new(Some(self.chats()));
        self.imp().chat_view.set_model(Some(&selection_model));
    }

    fn setup_callbacks(&self) {
        // Setup callback for activation of the entry
        self.imp()
            .entry
            .connect_activate(clone!(@weak self as window => move |_| {
                window.new_chat();
            }));

        // Setup callback for clicking (and the releasing) the icon of the entry
        self.imp().entry.connect_icon_release(
            clone!(@weak self as window => move |_,_| {
                window.new_chat();
            }),
        );
    }

    fn new_chat(&self) {
        // create client
        let client = APIClient::new(std::env::var("API_KEY").expect("API_KEY var doesn't exist"));
        // Get content from entry and clear it
        let buffer = self.imp().entry.buffer();
        let content = buffer.text().to_string();
        if content.is_empty() { return };
        buffer.set_text("");
        
        // Add new chat to model
        let chat = ChatObject::new(false, content.clone());
        self.chats().append(&chat);

        // handle api call
        let (sender, receiver) = async_channel::bounded(1);
        let shared_self = Arc::new(Mutex::new(self.clone()));
        let shared_receiver: Receiver<Result<Response, Error>> = receiver.clone();
        
        // for async actions in gtk
        runtime().spawn(clone!(@strong sender => async move {
            let response = client.send_chat_message(&content).await;
            sender.send(response).await.expect("The channel needs to be open");
        }));
        // The main loop executes the asynchronous block [try to cut this down a lot / organize into other mod if possible]
        glib::spawn_future_local(async move {
            while let Ok(response) = shared_receiver.recv().await {
                if let Ok(response) = response {
                    println!("{:#?}", response);
                    match response.status() {
                        reqwest::StatusCode::OK => {
                            match response.text().await {
                                Ok(body) => {
                                    println!("{:#?}", body);
                                    match serde_json::from_str::<APIResponse>(&body) {
                                        Ok(api_response) => {
                                            let text = &api_response.content[0].text;
                                            let incoming_chat = ChatObject::new(true, text.to_string());
                                            if let Ok(guard) = shared_self.lock() {
                                                guard.chats().append(&incoming_chat);
                                            } else {
                                                println!("Failed to acquire lock on shared_self");
                                            }
                                        }
                                        Err(e) => {
                                            eprintln!("Failed to deserialize response: {}", e);
                                        }
                                    }
                                }
                                Err(e) => {
                                    println!("Bad request: {}", e);
                                }
                            }
                        }
                        reqwest::StatusCode::UNAUTHORIZED => {
                            println!("Need to grab a new token");
                        }
                        other => {
                            panic!("Uh oh! Something unexpected happened: {:?}", other);
                        }
                    }
                } else {
                    println!("Could not make a GET request");
                }
            }
        });
    }

    fn setup_factory(&self) {
        // Create a new factory
        let factory = SignalListItemFactory::new();

        // Create an empty `chatRow` during setup
        factory.connect_setup(move |_, list_item| {
            // Create `chatRow`
            let chat_row = ChatRow::new();
            list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .set_child(Some(&chat_row));
        });

        // Tell factory how to bind `chatRow` to a `chatObject`
        factory.connect_bind(move |_, list_item| {
            // Get `chatObject` from `ListItem`
            let chat_object = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .item()
                .and_downcast::<ChatObject>()
                .expect("The item has to be an `ChatObject`.");

            // Get `chatRow` from `ListItem`
            let chat_row = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .child()
                .and_downcast::<ChatRow>()
                .expect("The child has to be a `ChatRow`.");

            chat_row.bind(&chat_object);

            if chat_object.incoming() {
                chat_row.set_valign(gtk::Align::Start);
            } else {
                chat_row.set_halign(gtk::Align::End);
            }
        });

        // Tell factory how to unbind `chatRow` from `chatObject`
        factory.connect_unbind(move |_, list_item| {
            // Get `chatRow` from `ListItem`
            let chat_row = list_item
                .downcast_ref::<ListItem>()
                .expect("Needs to be ListItem")
                .child()
                .and_downcast::<ChatRow>()
                .expect("The child has to be a `ChatRow`.");

            chat_row.unbind();
        });

        // Set the factory of the list view
        self.imp().chat_view.set_factory(Some(&factory));
    }
}