use std::sync::Arc;

use crate::{error::AppErroFFI, storage::redb_storage::RedbStorage};

mod error;
mod key_value_storage;
mod preferences;
mod redb_storage;

type PreferencesInner = preferences::Preferences<RedbStorage>;

pub struct Preferences {
    inner: Arc<PreferencesInner>,
}

#[boltffi::export]
impl Preferences {
    pub fn open(path: &str) -> Result<Self, AppErroFFI> {
        let storage = RedbStorage::open(path).map_err(|_| AppErroFFI::StorageError)?;
        println!("DEBUG: open");
        let inner = Arc::new(PreferencesInner::new(storage));
        Ok(Self { inner })
    }

    pub fn set_string(&self, key: &str, value: String) -> Result<(), AppErroFFI> {
        println!("DEBUG: set_string");
        self.inner
            .set_string(key, value)
            .map_err(|_| AppErroFFI::StorageError)
    }

    pub fn get_string(&self, key: &str) -> Result<Option<String>, AppErroFFI> {
        println!("DEBUG: get_string");
        self.inner
            .get_string(key)
            .map_err(|_| AppErroFFI::StorageError)
    }

    pub fn remove(&self, key: &str) -> Result<(), AppErroFFI> {
        self.inner.remove(key).map_err(|_| AppErroFFI::StorageError)
    }
}
