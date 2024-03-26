mod window;
mod chat_object;
mod chat_row;

use dotenv::dotenv;

use gtk::prelude::*;
use gtk::{glib, gio, Application};
use window::Window;

const APP_ID: &str = "org.gtk_rs.HelloWorld";

fn main() -> glib::ExitCode {
    dotenv().ok();
    // Register and include resources
    gio::resources_register_include!("compiled.gresource")
        .expect("Failed to register resources.");

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create a window and set the title
    let window = Window::new(app);
    window.set_default_size(600, 600);

    // Present window
    window.present();
}