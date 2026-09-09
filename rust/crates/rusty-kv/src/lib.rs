pub mod error;

use std::path::Path;

use redb::{Database, ReadableDatabase, TableDefinition, backends::InMemoryBackend};
use serde::{Serialize, de::DeserializeOwned};

use crate::error::KeyValueError;

const TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("key_value_table");

pub struct KeyValue {
    db: Database,
}

impl KeyValue {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, KeyValueError> {
        let db = Database::create(path).map_err(map_redb_error)?;
        Ok(Self { db })
    }

    pub fn in_memory() -> Result<Self, KeyValueError> {
        let db = Database::builder()
            .create_with_backend(InMemoryBackend::new())
            .map_err(map_redb_error)?;
        Ok(Self { db })
    }
}

impl KeyValue {
    pub fn get<T>(&self, key: impl AsRef<str>) -> Result<Option<T>, KeyValueError>
    where
        T: DeserializeOwned,
    {
        let txn = self.db.begin_read().map_err(map_redb_error)?;

        let table = match txn.open_table(TABLE) {
            Ok(table) => table,
            Err(redb::TableError::TableDoesNotExist(_)) => return Ok(None),
            Err(e) => return Err(map_redb_error(e)),
        };

        table
            .get(key.as_ref())
            .map_err(map_redb_error)?
            .map(|guard| serde_json::from_slice(guard.value()))
            .transpose()
            .map_err(KeyValueError::Serialization)
    }

    pub fn set<T>(&self, key: impl AsRef<str>, value: &T) -> Result<(), KeyValueError>
    where
        T: Serialize,
    {
        let bytes = serde_json::to_vec(value)?;
        let txn = self.db.begin_write().map_err(map_redb_error)?;

        {
            let mut table = txn.open_table(TABLE).map_err(map_redb_error)?;
            table
                .insert(key.as_ref(), bytes.as_slice())
                .map_err(map_redb_error)?;
        }

        txn.commit().map_err(map_redb_error)?;
        Ok(())
    }

    pub fn remove(&self, key: impl AsRef<str>) -> Result<bool, KeyValueError> {
        let txn = self.db.begin_write().map_err(map_redb_error)?;

        let removed = {
            let mut table = txn.open_table(TABLE).map_err(map_redb_error)?;
            table
                .remove(key.as_ref())
                .map_err(map_redb_error)?
                .is_some()
        };

        if removed {
            txn.commit().map_err(map_redb_error)?;
        }

        Ok(removed)
    }
}

fn map_redb_error(err: impl std::error::Error + Send + Sync + 'static) -> KeyValueError {
    KeyValueError::Database {
        source: Box::new(err),
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use tempfile::NamedTempFile;

    use super::*;

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct User {
        name: String,
        age: u32,
    }

    #[test]
    fn get_and_set() {
        let file = NamedTempFile::new().unwrap();
        let kv = KeyValue::new(file.path()).unwrap();

        kv.set("key", &42).unwrap();
        let value = kv.get::<i32>("key").unwrap();
        assert_eq!(value, Some(42));

        let user = User {
            name: String::from("Rusty"),
            age: 42,
        };

        kv.set("user", &user).unwrap();
        let value = kv.get::<User>("user").unwrap();
        assert_eq!(value, Some(user));
    }

    #[test]
    fn set_existing_key_overwrite() {
        let file = NamedTempFile::new().unwrap();
        let kv = KeyValue::new(file.path()).unwrap();

        kv.set("key", &42).unwrap();
        kv.set("key", &84).unwrap();
        let value = kv.get::<i32>("key").unwrap();
        assert_eq!(value, Some(84));
    }

    #[test]
    fn get_missing_key_return_none() {
        let file = NamedTempFile::new().unwrap();
        let kv = KeyValue::new(file.path()).unwrap();

        let value = kv.get::<i32>("key").unwrap();
        assert_eq!(value, None);
    }

    #[test]
    fn delete_missing_key_return_false() {
        let file = NamedTempFile::new().unwrap();
        let kv = KeyValue::new(file.path()).unwrap();

        let removed = kv.remove("key").unwrap();
        assert!(!removed);
    }

    #[test]
    fn delete_existing_key_return_true() {
        let file = NamedTempFile::new().unwrap();
        let kv = KeyValue::new(file.path()).unwrap();

        kv.set("key", &"value").unwrap();
        let removed = kv.remove("key").unwrap();
        assert!(removed);
    }

    #[test]
    fn in_memory() {
        let kv = KeyValue::in_memory().unwrap();
        let value = kv.get::<i32>("key").unwrap();
        assert_eq!(value, None);
    }
}
