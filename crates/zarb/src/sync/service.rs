use zarb_types::error::Result;
use crate::network::{r#trait::Network};
use super::{config::Config, firewall::Firewall};




pub struct Service {
    config: Config,
    firewall: Firewall,
    network: Box<dyn Network>,
}

impl Service {
    pub fn new(config: &Config, network: Box<dyn Network>) -> Result<Self> {
        Ok(Service {
            config: config.clone(),
            firewall: Firewall::new(&config.firewall)?,
            network
        })
    }

    
}