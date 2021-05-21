use gio::prelude::*;
use gtk::prelude::*;
use gtk_macros::{ /* get_widget ,*/ action };
use glib::clone;

use std::env;

use crate::constants::{ APP_ID, WINDOW_UI, /* EDITOR_SQL_UI */ };

// use crate::ui::providers::{ Providers };


#[derive(Copy, Clone)]
enum AppEvent {
    FileNew,
    Quit,
    ConnectionAdd,
    // ConnectionRemove,
    // ConnectionEdit
}

pub struct MainWindow {
    app: gtk::Application,
    window: gtk::ApplicationWindow,
    // sender: glib::Sender<AppEvent>,
    // receiver: glib::Receiver<AppEvent>
}

impl MainWindow {

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
            // sender: sender,
            // receiver: receiver
        };

        application.attach_signal_handlers(&sender);
        application.setup_actions(&sender);
        application.setup_receiver(receiver);

        return Ok(application);
    }

    pub fn run(&self) {
        let args: Vec<String> = env::args().collect();
        self.app.run(&args);
    }

    fn setup_actions(&self, sender: &glib::Sender<AppEvent>) {
        action!(self.app, "file.new",
            clone!(
                @strong sender as s 
                => move |_, _| {
                    if let Err(e) = s.send(AppEvent::FileNew) {
                        println!("{:?}", e);
                    }
                }
            )
        );

        action!(self.app, "quit",
            clone!(
                @strong sender as s
                => move |_, _| {
                    if let Err(e) = s.send(AppEvent::Quit) {
                        println!("{:?}", e);
                    }
                }
            )
        );

        // connections
        action!(self.app, "connections.add",
            clone!(
                @strong sender as s
                => move |_, _| {
                    if let Err(e) = s.send(AppEvent::ConnectionAdd) {
                        println!("{:?}", e);
                    }
                }
            )
        );

        // action!(self.app, "connections.remove",
        //     clone!(@strong self.sender as sender => move |_, _| {
        //         if let Err(e) = sender.send(AppEvent::ConnectionRemove) {
        //             println!("{:?}", e);
        //         }
        //     })
        // );

        // action!(self.app, "connections.edit",
        //     clone!(@strong self.sender as sender => move |_, _| {
        //         if let Err(e) = sender.send(AppEvent::ConnectionEdit) {
        //             println!("{:?}", e);
        //         }
        //     })
        // );
    }

    fn setup_receiver(&self, receiver: glib::Receiver<AppEvent>) {
        let mut remain_open = true;
        receiver.attach(
            None,
            clone!(
                // @strong self as application,
                // @strong self.sender as sender, 
                @strong self.app as app, 
                @strong self.window as window => move |event| {

                match event {
                    AppEvent::FileNew => {
                        println!("file new");
                    }
                    AppEvent::Quit => {
                        println!("quit");
                        // close this channel
                        remain_open = false;
                        app.quit();
                    }
                    AppEvent::ConnectionAdd => {
                        println!("connection add");
                        // application.show_connection_add_window();
                        // show_connection_add_window();
                    }
                    // AppEvent::ConnectionRemove => {
                    //     println!("connection remove");
                    // }
                    // AppEvent::ConnectionEdit => {
                    //     println!("connection edit");
                    // }
                }

                return glib::Continue(remain_open);
            })
        );
    }

    fn attach_signal_handlers(&self, sender: &glib::Sender<AppEvent>) {
        self.app.connect_activate(
            clone!(@weak self.window as window => move |app| {
                window.set_application(Some(app));
                app.add_window(&window);
                window.present();
            })
        );

        self.window.connect_destroy(
            clone!(
                @strong sender as s
                => move |_| {
                    if let Err(e) = s.send(AppEvent::Quit) {
                        println!("{:?}", e);
                    }
                } 
            )
        );
    }

    // fn show_connection_providers(&self) {
    //     let providers = Providers {};
    //     providers.show();
    // }

    fn show_connection_add_window(&self) {
        println!("show_connection_add_window");
    }
}