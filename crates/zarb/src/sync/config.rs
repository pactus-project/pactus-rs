use std::time::Duration;

use crate::sync::firewall;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub moniker: String,
    // TODO: save/load as string like: "5s"
    pub heartbeat_timeout: Duration,
    pub firewall: firewall::config::Config,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            moniker: "".to_string(),
            heartbeat_timeout: Duration::from_secs(5),
            firewall: firewall::config::Config::default(),
        }
    }
}
