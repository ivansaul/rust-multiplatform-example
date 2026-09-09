use serde::{Deserialize, Serialize};

#[boltffi::data]
#[derive(Debug, Clone, Copy)]
pub enum CoreColorScheme {
    Light,
    Dark,
}

#[boltffi::data]
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub enum ColorSchemeSelection {
    Light,
    Dark,
    #[default]
    System,
}

impl From<&str> for ColorSchemeSelection {
    fn from(value: &str) -> Self {
        match value {
            "Light" | "light" => ColorSchemeSelection::Light,
            "Dark" | "dark" => ColorSchemeSelection::Dark,
            _ => ColorSchemeSelection::System,
        }
    }
}

// impl From<String> for ColorSchemeSelection {
//     fn from(value: String) -> Self {
//         value.as_str().into()
//     }
// }

impl From<ColorSchemeSelection> for String {
    fn from(value: ColorSchemeSelection) -> Self {
        value.into()
    }
}
