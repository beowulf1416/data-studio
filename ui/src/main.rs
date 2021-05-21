#![forbid(unsafe_code)]

extern crate gtk;
extern crate gio;
extern crate glib;

// extern crate clap;
use clap::{Arg, App /*, SubCommand */ };

mod constants;
mod ui;

use std::io::{Error, ErrorKind};
use std::env;
use glib::Bytes;
use gio::{resources_register, Resource};

use common::application::Application;
use common::appconfig::ApplicationConfiguration;

use ui::main_window::MainWindow;


fn initialize() -> Result<(), Box<dyn std::error::Error>> {
    let data: &[u8] = include_bytes!(concat!(
        env!("OUT_DIR"),
        "/resources.gresource"
    ));
    let gbytes = Bytes::from_static(data.as_ref());
    let resource = Resource::from_data(&gbytes).unwrap();
    resources_register(&resource);

    return Ok(());
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let matches = App::new("data-studio")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("<file>")
            .help("use specified configuration file"))
        .get_matches();

    let cfg = matches.value_of("config").unwrap_or("config.json");

    let mut settings = config::Config::new();
    settings
        .merge(config::File::with_name(cfg)).unwrap();

    if let Ok(cfg) = settings.try_into::<ApplicationConfiguration>() {
        if let Ok(app) = Application::new(cfg) {
            if let Err(_) = gtk::init() {
                println!("unable to initialize gtk toolkit");
            }
        
            initialize()?;

            let main_window = MainWindow::new(app)?;
            main_window.run();

            return Ok(());
        } else {
            return Err(Box::new(Error::new(ErrorKind::Other, "Unable create application object")));
        }
    } else {
        return Err(Box::new(Error::new(ErrorKind::Other, "Unable to load configuration file")));
    }
}
