use std::path::Path;

use redb::{Database, ReadableDatabase, TableDefinition, TableError};

use crate::storage::{error::StorageError, key_value_storage::KeyValueStorage};

const PREFERENCES: TableDefinition<&str, &[u8]> = TableDefinition::new("preferences");

pub struct RedbStorage {
    db: Database,
}

impl RedbStorage {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, StorageError> {
        let db = Database::create(path).map_err(|e| StorageError::Operation {
            context: "Failed to create database".into(),
            source: Box::new(e),
        })?;
        Ok(Self { db })
    }
}

impl KeyValueStorage for RedbStorage {
    fn get(&self, key: &str) -> Result<Option<Vec<u8>>, StorageError> {
        let txn = self.db.begin_read().map_err(|e| StorageError::Operation {
            context: "Failed to begin_read transaction".into(),
            source: Box::new(e),
        })?;

        let table = match txn.open_table(PREFERENCES) {
            Ok(table) => table,
            Err(TableError::TableDoesNotExist(_)) => {
                return Ok(None);
            }
            Err(e) => {
                return Err(StorageError::Operation {
                    context: "Failed to open table".into(),
                    source: Box::new(e),
                });
            }
        };

        let value = table.get(key).map_err(|e| StorageError::Operation {
            context: format!("Failed to read key '{key}'"),
            source: Box::new(e),
        })?;

        Ok(value.map(|guard| guard.value().to_vec()))
    }

    fn set(&self, key: &str, value: &[u8]) -> Result<(), StorageError> {
        let txn = self.db.begin_write().map_err(|e| StorageError::Operation {
            context: "Failed to begin write transaction".into(),
            source: Box::new(e),
        })?;
        {
            let mut table = txn
                .open_table(PREFERENCES)
                .map_err(|e| StorageError::Operation {
                    context: "Failed to open table for writing".into(),
                    source: Box::new(e),
                })?;

            table
                .insert(key, value)
                .map_err(|e| StorageError::Operation {
                    context: format!("Failed to insert key '{key}'"),
                    source: Box::new(e),
                })?;
        }

        txn.commit().map_err(|e| StorageError::Operation {
            context: "Failed to commit write transaction".into(),
            source: Box::new(e),
        })?;

        Ok(())
    }

    fn remove(&self, key: &str) -> Result<(), StorageError> {
        let txn = self.db.begin_write().map_err(|e| StorageError::Operation {
            context: "Failed to begin write transaction".into(),
            source: Box::new(e),
        })?;
        {
            let mut table = txn
                .open_table(PREFERENCES)
                .map_err(|e| StorageError::Operation {
                    context: "Failed to open table for writing".into(),
                    source: Box::new(e),
                })?;
            table.remove(key).map_err(|e| StorageError::Operation {
                context: "Failed to remove key '{key}'".into(),
                source: Box::new(e),
            })?;
        }

        txn.commit().map_err(|e| StorageError::Operation {
            context: "Failed to commit transaction".into(),
            source: Box::new(e),
        })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn set_and_get() {
        let file = NamedTempFile::new().unwrap();
        let storage = RedbStorage::open(file.path()).unwrap();

        storage.set("key", b"value").unwrap();

        let value = storage.get("key").unwrap();

        assert_eq!(value, Some(b"value".to_vec()));
    }

    #[test]
    fn get_missing_key_returns_none() {
        let file = NamedTempFile::new().unwrap();
        let storage = RedbStorage::open(file.path()).unwrap();

        let value = storage.get("missing").unwrap();

        assert_eq!(value, None);
    }

    #[test]
    fn set_overwrites_existing_value() {
        let file = NamedTempFile::new().unwrap();
        let storage = RedbStorage::open(file.path()).unwrap();

        storage.set("name", b"Jhon").unwrap();
        storage.set("name", b"Doe").unwrap();

        let value = storage.get("name").unwrap();
        assert_eq!(value, Some(b"Doe".to_vec()));
    }

    #[test]
    fn remove_value() {
        let file = NamedTempFile::new().unwrap();
        let storage = RedbStorage::open(file.path()).unwrap();

        storage.set("key", b"value").unwrap();
        storage.remove("key").unwrap();

        let value = storage.get("key").unwrap();

        assert_eq!(value, None);
    }
}
