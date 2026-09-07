use std::error::Error;

#[derive(Debug, thiserror::Error)]
pub enum TaskRepositoryError {
    #[error("Unavailable")]
    Unavailable,

    #[error("Internal error")]
    Internal {
        #[source]
        source: Box<dyn Error>,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum TaskServiceError {
    #[error("Not found")]
    NotFound,

    #[error("Unavailable")]
    Unavailable,

    #[error("Internal error")]
    Unexpected {
        #[source]
        source: Box<dyn Error>,
    },
}
