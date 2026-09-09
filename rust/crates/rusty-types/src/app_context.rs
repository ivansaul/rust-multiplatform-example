#[boltffi::data]
#[derive(Debug)]
pub struct AppContext {
    pub storage: Storage,
}

#[boltffi::data]
#[derive(Debug)]
pub struct Storage {
    pub app_support: String,
    pub cache: String,
    pub documents: String,
}
