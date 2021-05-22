use gio::prelude::*;
use gtk::prelude::*;
use gtk_macros::{ /* get_widget ,*/ action };
use glib::clone;

use std::env;

use crate::constants::{ APP_ID, WINDOW_UI, /* EDITOR_SQL_UI */ };

use crate::ui::providers::{ ProviderDialog };

use common::application::Application;
use common::appconfig::ApplicationConfiguration;


#[derive(Copy, Clone)]
enum AppEvent {
    FileNew,
    Quit,
    ConnectionAdd,
    // ConnectionRemove,
    // ConnectionEdit
}


pub struct MainWindow {
    application: Option<Application>,
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
            // gio::ApplicationFlags::HANDLES_COMMAND_LINE
        ).unwrap();
        
        let builder = gtk::Builder::from_resource(WINDOW_UI);
        let window: gtk::ApplicationWindow = builder.get_object("window.main").unwrap();
        window.set_type_hint(gdk::WindowTypeHint::Normal);

        // let builder_tab = gtk::Builder::from_resource(EDITOR_SQL_UI);
        // let editors: gtk::Notebook = builder.get_object("window.main.editors").unwrap();
        // editors.append_page(
        //     builder_tab.get_object("window.main.editors.sql"),
        //     Some("test")
        // );

        let (sender, receiver) = glib::MainContext::channel::<AppEvent>(glib::PRIORITY_DEFAULT);

        let main_window = Self {
            application: None,
            app: gtk_app,
            window: window,
            // sender: sender,
            // receiver: receiver
        };

        main_window.attach_signal_handlers(&sender);
        main_window.setup_actions(&sender);
        main_window.setup_receiver(receiver);


        gtk_app.add_main_option(
            "config",
            glib::Char::new('c').unwrap(),
            glib::OptionFlags::IN_MAIN,
            glib::OptionArg::Filename,
            "configuration file",
            Some("configuration file")
        );

        gtk_app.connect_handle_local_options( move |_, args| {
            if args.contains("config") {
                // https://valadoc.org/glib-2.0/GLib.VariantType.html
                match glib::VariantTy::new("*") {
                    Ok(v) => {
                        if let Some(cfg) = args.lookup_value("config", Some(&v)) {
                            let config_file = cfg.to_string()
                                .replace("b\'", "")
                                .replace("'", "");

                            let mut settings = config::Config::new();
                            settings
                                .merge(config::File::with_name(&config_file)).unwrap();

                            if let Ok(appcfg) = settings.try_into::<ApplicationConfiguration>() {
                                if let Ok(app) = Application::new(appcfg) {
                                    main_window.set_application_object(app);
                                } else {
                                    println!("Unable to create application object");
                                }
                            } else {
                                println!("unable to parse configuration file");
                            }
                        }
                    }
                    Err(e) => {
                        println!("error {:?}", e);
                    }
                }
            }
            return -1;
        });

        return Ok(main_window);
    }

    fn set_application_object(&self, application: Application) {
        self.application = Some(application);

        // return Ok(self);
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
                @strong self.application as application,
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
                        let w = window.clone();
                        if let Some(a) = application {
                            if let Ok(dialog) = ProviderDialog::new(&a) {
                                let result = dialog.show(&w.upcast::<gtk::Window>());
                                println!("response {:?}", result);
                            } else {
                                println!("unable to show provider dialog");
                            }
                        } else {
                            println!("application object not yet initialized");
                        }
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
        // self.app.connect_command_line(
        //     move |_,args| {
        //         println!("connect_command_line");
        //         println!("{:?}", args);

        //         return 1;
        //     }
        // );
        // self.connect_handle_local_options(move|_, args| {
        //     println!("{:?}", args);
        //     return -1;
        // });

        self.app.connect_activate(
            clone!(@weak self.window as window => move |app| {
                window.set_application(Some(app));
                app.add_window(&window);
                window.present();
            })
        );

        self.window.connect_destroy(
            clone!(
                @strong sender as s => move |_| {
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

    // fn show_connection_add_window(&self) {
    //     println!("show_connection_add_window");
    // }
}