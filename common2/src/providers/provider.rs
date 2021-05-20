pub trait Provider {
    fn register(&self);
    fn help(&self) -> Option<&str> {
        None
    }
}

pub trait ProviderRegistrar {
    fn register(&mut self, name: &str, function: Box<dyn Provider>);
}

pub struct ProviderDeclaration {
    pub rustc_version: &'static str,
    pub core_version: &'static str,
    pub register: unsafe extern "C" fn(&mut dyn ProviderRegistrar)
}