mod components;

extern crate log;

use log::{
    info
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

    gio::resources_register_include!("ds.gresource")
        .expect("Failed to register resources.");

    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    let model = components::Application::new(app);
    let relm = RelmApp::new(model);
    relm.run();

    info!("Shutting down");
}
