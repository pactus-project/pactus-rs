use super::message::*;
use crate::error::{Error, Result};
use libp2p::PeerId;
use minicbor::{bytes::ByteVec, Decode, Encode};
use std::vec::Vec;

#[derive(Debug)]
pub struct Bundle {
    pub initiator: PeerId,
    pub message: Box<dyn Message>,
}

#[derive(Encode, Decode)]
#[cbor(map)]
struct RawBundle {
    #[n(1)]
    pub version: i32,
    #[n(2)]
    pub flags: i32,
    #[n(3)]
    pub initiator_data: ByteVec,
    #[n(4)]
    pub message_type: Type,
    #[n(5)]
    pub message_data: ByteVec,
}

impl Bundle {
    pub fn new(initiator: PeerId, message: Box<dyn Message>) -> Result<Self> {
        Ok(Self { initiator, message })
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let raw: RawBundle = minicbor::decode(data)?;
        let initiator = PeerId::from_bytes(&raw.initiator_data)
            .map_err(|err| Error::DecodeError(err.to_string()))?;
        let msg: Result<Box<dyn Message>> = match raw.message_type {
            Type::Hello => Ok(Box::new(minicbor::decode::<hello::HelloMessage>(
                raw.message_data.as_ref(),
            )?)),
            Type::Heartbeat => Ok(Box::new(minicbor::decode::<heartbeat::HeartbeatMessage>(
                raw.message_data.as_ref(),
            )?)),
            _ => Err(Error::InvalidMessage(format!(
                "message type {} not supported yet",
                raw.message_type
            ))),
        };

        Self::new(initiator, msg?)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let initiator_data = self.initiator.to_bytes().into();
        let message_data = self.message.to_bytes()?.into();
        let raw = RawBundle {
            version: 1,
            flags: 0,
            initiator_data,
            message_type: self.message.message_type(),
            message_data,
        };

        Ok(minicbor::to_vec(raw)?)
    }

    pub fn basic_check(&self) -> Result<()> {
        self.message.basic_check()
    }

    pub fn message_type(&self) -> Type {
        self.message.message_type()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decoding() {
        let buf = hex::decode(
            "a50101020003582212200860eba8c1303d54000000000000000000000000000000000000000000000000040105589fa6016a302e392e302d626574610263666f6f035860b945dd77f99573a3db273ed1b720c7624deacd5ac19ea32e41eac7901122826e8cc1bc9e170e516c6b52c31146441c3d0fe7772ac1ae28b47faae556fa99b9fedd95c07f4cbc84037845eb35879a1e48c34a56139152d1c8f0baffeafba8b02c04582010f34183da6829a5b6449ce7543a8c356233bdaceed6d31c3bf6dd8ac0029fb605190159061858",
        ).unwrap();

        let msg = Bundle::from_bytes(buf.as_slice()).unwrap();
        assert_eq!(buf, msg.to_bytes().unwrap());
    }
}
