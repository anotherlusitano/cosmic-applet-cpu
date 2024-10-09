use cosmic::cosmic_config::{self, cosmic_config_derive::CosmicConfigEntry, CosmicConfigEntry};

#[derive(Clone, CosmicConfigEntry, Debug, Eq, PartialEq)]
pub struct Config {
    pub refresh_time: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self { refresh_time: 2 }
    }
}
