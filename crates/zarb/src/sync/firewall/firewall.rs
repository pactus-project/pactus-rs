use super::{config, config::Config};
use crate::sync::bundle;
use crate::{error::Result, sync::bundle::bundle::Bundle};

pub(crate) struct Firewall {
    config: Config,
}

impl Firewall {
    pub fn new(config: &Config) -> Result<Self> {
        Ok(Firewall {
            config: config.clone(),
        })
    }

    pub fn open_bundle(&self, data: &[u8]) -> Result<Bundle> {
        Bundle::from_bytes(data)
    }
}
