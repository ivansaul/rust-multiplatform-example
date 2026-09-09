pub mod error;

use std::sync::Arc;

use rusty_types::color_scheme::ColorSchemeSelection;

use crate::{
    ffi::settings::error::SettingsError, settings::SettingsService as CoreSettingsService,
};

#[derive(Clone)]
pub struct SettingsService {
    inner: Arc<CoreSettingsService>,
}

impl SettingsService {
    pub fn new(inner: Arc<CoreSettingsService>) -> Self {
        Self { inner }
    }
}

#[boltffi::export]
impl SettingsService {
    pub fn color_scheme(&self) -> ColorSchemeSelection {
        self.inner.get_color_scheme()
    }

    pub fn update_color_scheme(&self, scheme: ColorSchemeSelection) -> Result<(), SettingsError> {
        self.inner
            .set_color_scheme(scheme)
            .map_err(SettingsError::from)
    }
}
