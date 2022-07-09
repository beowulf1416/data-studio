mod components;

extern crate log;

use log::{
    info,
    debug,
    error
};

// use relm4::{
//     RelmApp,
//     gtk::{
//         gio,
//         Application
//     }
// };
use gtk::{
    prelude::*,
    gio,
    Application, 
    CssProvider,
    StyleContext,
    gdk::Display
};

use crate::components::main_window::MainWindow;


const APP_ID: &str = "org.tomale.ds";

fn main() {
    env_logger::init();

    info!("Starting up");

    if let Err(e) = gtk::init() {
        error!("Unable to initialize gtk: {:?}", e);
    }

    gio::resources_register_include!("gds.gresource")
        .expect("Failed to register resources.");

    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    // let model = components::Application::new(&app);
    // // let relm = RelmApp::new(model);
    // let relm = RelmApp::with_app(model, app);
    // relm.run();

    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);
    app.run();

    info!("Shutting down");
}

fn build_ui(app: &gtk::Application) {
    let window = MainWindow::new(app);
    window.show();
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_resource("/org/tomale/ds//style.css");

    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display"), 
        &provider, 
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
    );
}