use std::{ env, fs };

use crate::providers::registry::{ Registry };

pub struct Application {
    registry: Registry,
}

impl Application {

    pub fn new() -> Self {
        println!("Application::new()");

        let application = Self {
            registry: Registry::new()
        };

        let current_dir = env::current_dir();
        println!("{:?}", current_dir);

        let current_exe = env::current_exe().unwrap();
        let mut path = current_exe;
        path.pop();
        println!("{:?}", path);

        for entry in fs::read_dir(path).unwrap() {
            let f = entry.unwrap();
            let fname = f.file_name();
            println!("{:?}", fname);
        }
        
        return application;
    }
}