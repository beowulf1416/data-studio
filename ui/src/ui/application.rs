use gio::prelude::*;
use gtk::prelude::*;
// use gtk_macros::{ get_widget };
use glib::clone;

use std::env;

use crate::constants::{ APP_ID, WINDOW_UI, EDITOR_SQL_UI };

pub struct Application {
    app: gtk::Application,
    window: gtk::ApplicationWindow
}

impl Application {

    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let gtk_app = gtk::Application::new(
            Some(APP_ID),
            gio::ApplicationFlags::FLAGS_NONE
        ).unwrap();
        
        let builder = gtk::Builder::from_resource(WINDOW_UI);
        let window: gtk::ApplicationWindow = builder.get_object("window.main").unwrap();

        let builder_tab = gtk::Builder::from_resource(EDITOR_SQL_UI);
        let editors: gtk::Notebook = builder.get_object("window.main.editors").unwrap();
        // editors.append_page(
        //     builder_tab.get_object("window.main.editors.sql"),
        //     Some("test")
        // );

        // gtk_app.add_window(&window);

        let application = Self {
            app: gtk_app,
            window: window
        };

        application.attach_signal_handlers();

        return Ok(application);
    }

    pub fn run(&self) {
        let args: Vec<String> = env::args().collect();
        self.app.run(&args);
    }

    fn attach_signal_handlers(&self) {
        self.app.connect_activate(
            clone!(@weak self.window as window => move |app| {
                window.set_application(Some(app));
                app.add_window(&window);
                window.present();
            })
        );
    }
}