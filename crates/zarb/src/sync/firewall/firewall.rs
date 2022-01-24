use super::{config, config::Config};
use crate::sync::message;
use crate::{error::Result, sync::message::message::Message};

pub(crate) struct Firewall {
    config: Config,
}

impl Firewall {
    pub fn new(config: &Config) -> Result<Self> {
        Ok(Firewall {
            config: config.clone(),
        })
    }

    pub fn open_message(&self, data: &[u8]) -> Result<Message> {
        Message::from_bytes(data)
    }
}
