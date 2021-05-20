use serde::{ Deserialize };

#[derive(Deserialize, Debug)]
pub struct Plugin {
    pub name: String
}

#[derive(Deserialize, Debug)]
pub struct ApplicationConfiguration {
    pub plugins: Vec<Plugin>
}