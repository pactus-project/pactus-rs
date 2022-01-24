use crate::{network, sync};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub network: network::config::Config,
    pub sync: sync::config::Config,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            network: network::config::Config::default(),
            sync: sync::config::Config::default(),
        }
    }
}
