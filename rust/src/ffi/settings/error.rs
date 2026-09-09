use crate::settings::error::SettingsError as CoreSettingsError;

#[boltffi::error]
#[derive(Debug, Clone, thiserror::Error)]
pub enum SettingsError {
    #[error("storage error")]
    Storage,
}

impl From<CoreSettingsError> for SettingsError {
    fn from(error: CoreSettingsError) -> Self {
        match error {
            CoreSettingsError::Storage(_) => SettingsError::Storage,
        }
    }
}
