use rusty_types::color_scheme::ColorSchemeSelection;

use crate::settings::{SettingsService, error::SettingsError, global_config::GlobalConfigKey};

impl SettingsService {
    pub fn set_color_scheme(&self, scheme: ColorSchemeSelection) -> Result<(), SettingsError> {
        self.kv
            .set(GlobalConfigKey::ColorScheme, &scheme)
            .map_err(SettingsError::from)
    }

    pub fn get_color_scheme(&self) -> ColorSchemeSelection {
        self.kv
            .get(GlobalConfigKey::ColorScheme)
            .ok()
            .flatten()
            .unwrap_or_default()
    }
}
