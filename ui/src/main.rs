#![forbid(unsafe_code)]

extern crate gtk;
extern crate gio;
extern crate glib;

mod constants;
mod ui;

use std::env;
use glib::Bytes;
use gio::{resources_register, Resource};

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
    if let Err(_) = gtk::init() {
        println!("unable to initialize gtk toolkit");
    }

    initialize()?;

    // for (key, value) in env::vars() {
    //     println!("{}: {}", key, value);
    // }

    let main_window = MainWindow::new()?;
    main_window.run();

    return Ok(());
}
