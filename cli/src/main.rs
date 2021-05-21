extern crate config;
// extern crate common;

extern crate clap;
use clap::{Arg, App /*, SubCommand */ };

use common::application::Application;
use common::appconfig::ApplicationConfiguration;



fn main() {
    println!("Hello, world!");

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

    let cfg = settings.try_into::<ApplicationConfiguration>().unwrap();
    if let Ok(app) = Application::new(cfg) {
        println!("{:?}", app.providers());
    }
}
