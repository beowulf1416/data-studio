pub struct ProviderInfo {
    pub name: String,
    pub version: String,
}

pub trait Provider {
    fn info(self) -> ProviderInfo;
    fn register(self);
}