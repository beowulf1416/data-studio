// extern crate config;
extern crate common;

// use std::collections::HashMap;

use common::application::Application;


fn main() {
    println!("Hello, world!");

    // let mut settings = config::Config::new();
    // settings
    //     .merge(config::File::with_name("config")).unwrap()
    //     .merge(config::Environment::with_prefix("DS")).unwrap();

    // println!("{:?}", settings.try_into::<HashMap<String, String>>().unwrap());

    let app = Application::new();
    let providers = app.providers();
    for provider in providers {
        println!("{}", provider);
    }
}
