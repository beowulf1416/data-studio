use gio::prelude::*;
use gtk::prelude::*;
use gtk_macros::{ /* get_widget ,*/ action };
use glib::clone;

use std::env;

use crate::constants::{ APP_ID, WINDOW_UI, /* EDITOR_SQL_UI */ };

use crate::ui::providers::{ Providers };


enum AppEvent {
    FileNew,
    Quit,
    ConnectionAdd,
    ConnectionRemove,
    ConnectionEdit
}

pub struct Application {
    app: gtk::Application,
    window: gtk::ApplicationWindow,
    sender: glib::Sender<AppEvent>,
}

impl Application {

    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let gtk_app = gtk::Application::new(
            Some(APP_ID),
            gio::ApplicationFlags::FLAGS_NONE
        ).unwrap();
        
        let builder = gtk::Builder::from_resource(WINDOW_UI);
        let window: gtk::ApplicationWindow = builder.get_object("window.main").unwrap();

        // let builder_tab = gtk::Builder::from_resource(EDITOR_SQL_UI);
        // let editors: gtk::Notebook = builder.get_object("window.main.editors").unwrap();
        // editors.append_page(
        //     builder_tab.get_object("window.main.editors.sql"),
        //     Some("test")
        // );

        let (sender, receiver) = glib::MainContext::channel::<AppEvent>(glib::PRIORITY_DEFAULT);

        let application = Self {
            app: gtk_app,
            window: window,
            sender: sender
        };

        application.setup_actions();
        application.attach_signal_handlers();
        application.setup_receiver(receiver);

        return Ok(application);
    }

    pub fn run(&self) {
        let args: Vec<String> = env::args().collect();
        self.app.run(&args);
    }

    fn setup_actions(&self) {
        action!(self.app, "file.new",
            clone!(@strong self.sender as sender => move |_, _| {
                sender.send(AppEvent::FileNew).unwrap();
            })
        );

        action!(self.app, "quit",
            clone!(@strong self.sender as sender => move |_, _| {
                sender.send(AppEvent::Quit).unwrap();
            })
        );

        // connections
        action!(self.app, "connections.add",
            clone!(@strong self.sender as sender => move |_, _| {
                sender.send(AppEvent::ConnectionAdd).unwrap();
            }) 
        );

        action!(self.app, "connections.remove",
            clone!(@strong self.sender as sender => move |_, _| {
                sender.send(AppEvent::ConnectionRemove).unwrap();
            })
        );

        action!(self.app, "connections.edit",
            clone!(@strong self.sender as sender => move |_, _| {
                sender.send(AppEvent::ConnectionEdit).unwrap();
            })
        );
    }

    fn setup_receiver(&self, receiver: glib::Receiver<AppEvent>) {
        receiver.attach(
            None,
            clone!(
                // @strong self as this,
                @strong self.sender as sender, 
                @strong self.app as app, 
                @strong self.window as window => move |event| {

                match event {
                    AppEvent::FileNew => {
                        // println!("File New");
                        // this.show_connection_providers();
                        let providers = Providers {};
                        providers.show();
                    }
                    AppEvent::Quit => {
                        println!("quit");
                        app.quit();
                    }
                    AppEvent::ConnectionAdd => {
                        println!("connection add");
                    }
                    AppEvent::ConnectionRemove => {
                        println!("connection remove");
                    }
                    AppEvent::ConnectionEdit => {
                        println!("connection edit");
                    }
                }

                return glib::Continue(true);
            })
        );
    }

    fn attach_signal_handlers(&self) {
        self.app.connect_activate(
            clone!(@weak self.window as window => move |app| {
                window.set_application(Some(app));
                app.add_window(&window);
                window.present();
            })
        );

        self.window.connect_destroy(
            clone!(@strong self.sender as sender => move |_| {
                sender.send(AppEvent::Quit).unwrap();
            })
        );
    }

    // fn show_connection_providers(&self) {
    //     let providers = Providers {};
    //     providers.show();
    // }
}