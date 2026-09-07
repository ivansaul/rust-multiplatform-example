use std::path::PathBuf;

pub struct AppContext {
    pub storage: Storage,
}

pub struct Storage {
    pub app_support: PathBuf,
    pub cache: PathBuf,
    pub documents: PathBuf,
}
