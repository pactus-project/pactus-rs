use zarb_types::error::Result;

use crate::sync::message;

use super::{Config, config};

pub(crate) struct Firewall {
    config: Config,
}

impl Firewall {
    pub fn new(config: &Config) -> Result<Self> {
        Ok(Firewall{config: config.clone()})
    }

    pub fn open_message(&self, data: &[u8]) -> message::Message {
        todo!()
    }
}

