use serde::{Deserialize, Serialize};
use crate::{network, sync};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub network: network::Config,
    pub sync: sync::Config,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            network: network::Config::default(),
            sync: sync::Config::default(),
        }
    }
}
