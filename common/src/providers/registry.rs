// use std::collections::HashMap;
// use std::collections::hash_map::Keys;

use crate::providers::provider::{ Provider };


pub struct Registry {
    // providers: HashMap<String, Box<dyn Provider>>

}

impl Registry {

    pub fn new() -> Self {
        return Self {
            // providers: HashMap::default()
        }
    }

    // pub fn get_provider_names(self) -> Keys<'_, String> {
    //     return self.providers.keys();
    // }

    // pub fn add(self, provider: Box<dyn Provider>) {
    //     let info = provider.info();
    //     self.providers.insert(info.name, provider);
    // }
}