#[boltffi::error]
#[derive(Debug, Clone, thiserror::Error)]
pub enum CoreError {
    #[error("database error")]
    Database,

    #[error("internal error")]
    Internal,
}
