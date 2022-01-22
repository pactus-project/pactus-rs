use serde::{Deserialize, Serialize};
use crate::{network, sync};

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
