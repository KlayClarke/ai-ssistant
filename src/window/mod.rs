mod imp;

use gtk::subclass::prelude::*;
use glib::{Object, clone};
use gtk::{gio, glib, ListItem, SignalListItemFactory, prelude::*, NoSelection, Application};
use native_dialog::FileDialog;
use reqwest::{Error, Response};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::sync::{OnceLock, Arc, Mutex};
use tokio::runtime::Runtime;
use async_channel::Receiver;
use base64::{Engine as _, engine::general_purpose};


use crate::api_types::{APIResponse, ApiRequest, Block, ImageBlock, ImageSource, RequestBlock, RequestContent, TextBlock};
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

    fn current_chat(&self) -> ChatObject {
        self.imp()
            .current_chat
            .borrow()
            .clone()
            .expect("Could not get current chat")
    }

    fn chats(&self) -> gio::ListStore {
        self.imp()
            .chats
            .borrow()
            .clone()
            .expect("Could not get current chats.")
    }

    fn chats_vec(&self) -> Vec<ApiRequest> {
        self.imp()
            .chats_vec
            .borrow()
            .clone()
            .expect("Could not get chats_vec")
    }

    fn setup_chats(&self) {
        // Create new model
        let model = gio::ListStore::new::<ChatObject>();
        let model_: Vec<ApiRequest> = Vec::new();
        let current_chat: ChatObject = ChatObject::new("user".to_string(),"".to_string(), PathBuf::new());

        // Get state and set model
        self.imp().chats.replace(Some(model));
        self.imp().chats_vec.replace(Some(model_));
        self.imp().current_chat.replace(Some(current_chat));

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

        // setup callback for when text is entered in entry (we want to capture text and update current_chat object)
        self.imp().entry.connect_changed(clone!(@weak self as window => move |entry| {
            let buffer = entry.buffer();
            let content = buffer.text().to_string();
            window.current_chat().set_user_content(content);
        }));
        
        // Setup callback for clicking (and the releasing) the icon of the entry [CAN HANDLE IMAGE UPLOADS HERE]
        self.imp().entry.connect_icon_release(
            clone!(@weak self as window => move |_,_| {
                let file_path = window.handle_file_pick();
                window.current_chat().set_user_image(file_path);
            }),
        );
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

            if chat_object.role() == "assistant" {
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
    
    fn handle_file_pick(&self) -> PathBuf {
        FileDialog::new()
            .add_filter("Image", &["jpg", "png", "gif", "webp"])
            .set_location("~")
            .show_open_single_file()
            .unwrap()
            .unwrap()
    }

    fn process_image(&self, image: &PathBuf) -> String {
        let mut file = File::open(image).expect("Failed to open the image file");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).expect("Failed to read the image file");
        let base64_encoded = general_purpose::STANDARD.encode(buffer);

        base64_encoded
    }

    fn save_to_chats_vec(&self, current_chat: &ChatObject) {
        // extract chat data from chats and save in vec
        let role = current_chat.role().to_string();
        let content = current_chat.content().to_string();
        let image = current_chat.image();
        
        let request: ApiRequest;
        if image.exists() {
            // handle image content ApiRequest
            let base64_encoded = self.process_image(&image);
            request = ApiRequest {
                role,
                content: RequestContent::Blocks(vec![
                    Block::Image(RequestBlock {
                        image_block: ImageBlock {
                            type_: "image".to_string(),
                            source: ImageSource {
                                source_type: "base64".to_string(),
                                media_type: format!("image/{}", image.extension().unwrap().to_str().unwrap()).to_string(),
                                data: base64_encoded
                            }
                        },
                        text_block: TextBlock {
                            type_: "text".to_string(),
                            text: content
                        }
                    })
                ])
            };
        } else {
            // handle text content ApiRequest
            request = ApiRequest {
                role,
                content: RequestContent::Text(content)
            };
        }

        if let Some(chats_vec) = self.imp().chats_vec.borrow_mut().as_mut() {
            chats_vec.push(request);
        }

    }

    fn new_chat(&self) {
        // if entry empty, return
        if self.imp().entry.buffer().text().to_string().is_empty() { return }
        
        // create client
        let client = APIClient::new(std::env::var("API_KEY").expect("Failed to retrieve API_KEY environment variable!"));
        
        // get current chat
        let chat = ChatObject::new(self.current_chat().role().to_string(), self.current_chat().content().to_string(), self.current_chat().image());
        
        // Add new chat to model & convo
        self.chats().append(&chat);
        self.save_to_chats_vec(&chat);

        // get full convo
        let conversation = self.chats_vec();
        println!("{:?}", conversation);
        // handle api call
        let (sender, receiver) = async_channel::bounded(1);
        let shared_self = Arc::new(Mutex::new(self.clone()));
        let shared_receiver: Receiver<Result<Response, Error>> = receiver.clone();
        
        // for async actions in gtk
        runtime().spawn(clone!(@strong sender => async move {
            let response = client.send_chat_message(&conversation).await;
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
                                            let incoming_chat = ChatObject::new("assistant".to_string(), text.to_string(), PathBuf::new());
                                            if let Ok(guard) = shared_self.lock() {
                                                guard.chats().append(&incoming_chat);
                                                guard.save_to_chats_vec(&incoming_chat);
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

        // clear entry
        self.imp().entry.buffer().set_text("");

        // clear current chat
        self.imp().current_chat.replace(Some(ChatObject::new("user".to_string(),"".to_string(),PathBuf::new())));
    }
}