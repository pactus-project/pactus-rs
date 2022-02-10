use super::payload;
use crate::error::{Error, Result};
use libp2p::PeerId;
use minicbor::{bytes::ByteVec, Decode, Encode};
use std::vec::Vec;

#[derive(Debug)]
pub struct Message {
    pub initiator: PeerId,
    pub payload_type: payload::Type,
    pub payload_data: ByteVec,
    pub payload: Box<dyn payload::Payload>,
}

#[derive(Encode, Decode)]
#[cbor(map)]
pub struct RawMessage {
    #[n(1)]
    pub version: i32,
    #[n(2)]
    pub flags: i32,
    #[n(3)]
    pub initiator_data: ByteVec,
    #[n(4)]
    pub payload_type: payload::Type,
    #[n(5)]
    pub payload_data: ByteVec,
}

impl Message {
    pub fn new(
        initiator: PeerId,
        payload_type: payload::Type,
        payload_data: ByteVec,
        payload: Box<dyn payload::Payload>,
    ) -> Result<Self> {
        Ok(Self {
            initiator,
            payload_type,
            payload_data,
            payload,
        })
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let raw: RawMessage = minicbor::decode(data)?;
        let initiator = PeerId::from_bytes(&raw.initiator_data)
            .map_err(|err| Error::DecodeError(err.to_string()))?;
        let payload: Box<dyn payload::Payload> = match raw.payload_type {
            payload::Type::Salam => Box::new(minicbor::decode::<payload::salam::SalamPayload>(
                raw.payload_data.as_ref(),
            )?),
            payload::Type::Heartbeat => Box::new(minicbor::decode::<
                payload::heartbeat::HeartbeatPayload,
            >(raw.payload_data.as_ref())?),
            _ => {
                todo!()
            }
        };

        Self::new(initiator, raw.payload_type, raw.payload_data, payload)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let initiator_data = self.initiator.to_bytes().into();
        let payload_data = self.payload.to_bytes()?.into();
        let raw = RawMessage {
            version: 1,
            flags: 0,
            initiator_data,
            payload_type: self.payload.payload_type(),
            payload_data,
        };

        Ok(minicbor::to_vec(raw)?)
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

        let msg = Message::from_bytes(buf.as_slice()).unwrap();
        assert_eq!(buf, msg.to_bytes().unwrap());
    }
}
