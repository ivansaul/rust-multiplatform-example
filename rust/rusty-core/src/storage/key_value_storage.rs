use crate::storage::error::StorageError;

pub trait KeyValueStorage {
    fn get(&self, key: &str) -> Result<Option<Vec<u8>>, StorageError>;
    fn set(&self, key: &str, value: &[u8]) -> Result<(), StorageError>;
    fn remove(&self, key: &str) -> Result<(), StorageError>;
}
