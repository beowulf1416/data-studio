use common::providers::provider::{ ProviderInfo, Provider };

pub struct PostgreSQL {

}

impl Provider for PostgreSQL {

    fn info(self) --> ProviderInfo {
        return {
            name: "PostgreSQL",
            version: "0.0.1"
        }
    }
}