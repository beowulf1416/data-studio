use crate::appconfig::ApplicationConfiguration;

pub struct Application {
    pub configuration: ApplicationConfiguration
}

impl Application {

    pub fn new(configuration: ApplicationConfiguration) -> Result<Self, Box<dyn std::error::Error>> {
        let app = Self{
            configuration: configuration
        };

        return Ok(app);
    }

    pub fn providers(&self) -> Vec<String> {
        let mut providers = Vec::new();

        for plugin in self.configuration.plugins.iter() {
            providers.push(plugin.name.to_owned());
        }

        return providers;
    }
}