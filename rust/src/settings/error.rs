use rusty_kv::error::KeyValueError;

#[derive(Debug, thiserror::Error)]
pub enum SettingsError {
    #[error("database error")]
    Storage(#[from] KeyValueError),
}
