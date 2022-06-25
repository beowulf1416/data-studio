mod components;

extern crate log;

use log::{
    info,
    debug,
    error
};

use relm4::{
    RelmApp,
    gtk::{
        gio,
        Application
    }
};


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

    let model = components::Application::new(&app);
    // let relm = RelmApp::new(model);
    let relm = RelmApp::with_app(model, app);
    relm.run();

    info!("Shutting down");
}
