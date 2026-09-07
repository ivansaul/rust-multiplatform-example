use std::path::PathBuf;

use crate::app::context::AppContext as AppContextCore;
use crate::app::context::Storage as StorageCore;

#[boltffi::data]
pub struct Storage {
    pub app_support: String,
    pub cache: String,
    pub documents: String,
}

#[boltffi::data]
pub struct AppContext {
    pub storage: Storage,
}

impl From<Storage> for StorageCore {
    fn from(value: Storage) -> Self {
        Self {
            app_support: PathBuf::from(value.app_support),
            cache: PathBuf::from(value.cache),
            documents: PathBuf::from(value.documents),
        }
    }
}

impl From<AppContext> for AppContextCore {
    fn from(value: AppContext) -> Self {
        Self {
            storage: value.storage.into(),
        }
    }
}
