#[derive(Debug, Clone, Copy)]
pub enum GlobalConfigKey {
    ColorScheme,
}

impl From<GlobalConfigKey> for &'static str {
    fn from(key: GlobalConfigKey) -> Self {
        match key {
            GlobalConfigKey::ColorScheme => "color_scheme",
        }
    }
}

impl AsRef<str> for GlobalConfigKey {
    fn as_ref(&self) -> &str {
        match self {
            GlobalConfigKey::ColorScheme => "color_scheme",
        }
    }
}
