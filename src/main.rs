mod window;
mod chat_object;
mod chat_row;

use dotenv::dotenv;
use gtk::gdk::Display;
use gtk::{prelude::*, CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION};
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

    // load css
    app.connect_startup(|_| load_css());
    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("style.css"));

    gtk::style_context_add_provider_for_display(&Display::default().expect("Could not connect to a display"), &provider, STYLE_PROVIDER_PRIORITY_APPLICATION);
}

fn build_ui(app: &Application) {
    // Create a window and set the title
    let window = Window::new(app);
    window.set_default_size(600, 600);

    // Present window
    window.present();
}