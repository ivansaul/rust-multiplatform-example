use serde::{Deserialize, Serialize};

use crate::storage::error::StorageError;
use crate::storage::key_value_storage::KeyValueStorage;

#[derive(Serialize, Deserialize)]
enum StoredValue {
    String(String),
    Bool(bool),
    Int(i64),
    Double(f64),
}

pub struct Preferences<S>
where
    S: KeyValueStorage,
{
    storage: S,
}

impl<S> Preferences<S>
where
    S: KeyValueStorage,
{
    pub fn new(storage: S) -> Self {
        Self { storage }
    }

    pub fn set_string(&self, key: &str, value: String) -> Result<(), StorageError> {
        let value = StoredValue::String(value);

        let bytes = postcard::to_allocvec(&value).map_err(|_| StorageError::Serialization)?;

        self.storage.set(key, &bytes)?;

        Ok(())
    }

    pub fn get_string(&self, key: &str) -> Result<Option<String>, StorageError> {
        let Some(bytes) = self.storage.get(key)? else {
            return Ok(None);
        };

        let value: StoredValue =
            postcard::from_bytes(&bytes).map_err(|_| StorageError::Serialization)?;

        match value {
            StoredValue::String(value) => Ok(Some(value)),
            _ => Err(StorageError::WrongType),
        }
    }

    pub fn remove(&self, key: &str) -> Result<(), StorageError> {
        self.storage.remove(key)
    }
}
