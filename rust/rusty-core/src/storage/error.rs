use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("Storage operation failed: {context}: {source}")]
    Operation {
        context: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("Serialization failed")]
    Serialization,

    #[error("Stored value has an unexpected type")]
    WrongType,
}
