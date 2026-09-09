use rusty_kv::KeyValue;

pub mod appearance;
pub mod error;
pub mod global_config;

pub struct SettingsService {
    kv: KeyValue,
}

impl SettingsService {
    pub fn new(kv: KeyValue) -> Self {
        Self { kv }
    }
}
