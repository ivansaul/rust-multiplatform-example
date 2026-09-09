#[derive(Debug, thiserror:: Error)]
pub enum KeyValueError {
    #[error("Database error: {source}")]
    Database {
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error("Serialization/deserialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}
