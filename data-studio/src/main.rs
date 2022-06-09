extern crate log;

use log::{
    error
};

use relm4::{
    RelmApp,
    gtk::{
        Builder,
        builders::ApplicationBuilder
    },
    gtk::gio::MenuModel
};
    
mod models;
mod views;

use crate::models::application_model::ApplicationModel;

static APP_ID: &str = "com.tomale.gds";


fn main() {
    env_logger::init();

    if let Err(e) = relm4::gtk::init() {
        error!("An error occured during gtk init: {}", e);
    } else {
        let builder = Builder::from_string(include_str!("../resources/main.menu.ui"));
        let app_menu: MenuModel = builder.object("window.app.menu").expect("could not get application menu");

        let gtk_app = ApplicationBuilder::new()
            .application_id(APP_ID)
            .menubar(&app_menu)
            .build();

        let model = ApplicationModel::new();
        let app = RelmApp::with_app(model, gtk_app);
        app.run();
    }
}