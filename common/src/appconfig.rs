use serde::{ Deserialize };

#[derive(Deserialize, Debug, Clone)]
pub struct Plugin {
    pub name: String
}

#[derive(Deserialize, Debug, Clone)]
pub struct ApplicationConfiguration {
    pub plugins: Vec<Plugin>
}