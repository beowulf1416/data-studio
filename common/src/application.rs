
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
        
        return application;
    }
}