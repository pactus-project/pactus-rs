use super::payload;
use crate::crypto::public_key::PublicKey;
use crate::crypto::signature::Signature;
use crate::crypto::KeyPairType;
use crate::error::{Error, Result};
use crate::stamp::Stamp;
use minicbor::bytes::ByteVec;
use minicbor::{Decode, Encode};

#[derive(Debug)]
pub struct Transaction {
    pub stamp: Stamp,
    pub sequence: i32,
    pub fee: i64,
    pub memo: String,
    pub payload: Box<dyn payload::Payload>,
    pub public_key: Option<PublicKey>,
    pub signature: Option<Signature>,
}

#[derive(Encode, Decode)]
#[cbor(map)]
struct RawTransaction {
    #[n(1)]
    pub version: i32,
    #[n(2)]
    pub stamp: Stamp,
    #[n(3)]
    pub sequence: i32,
    #[n(4)]
    pub fee: i64,
    #[n(5)]
    pub payload_type: payload::Type,
    #[n(6)]
    pub payload_data: ByteVec,
    #[n(7)]
    pub memo: String,
    #[n(20)]
    pub public_key_data: Option<ByteVec>,
    #[n(21)]
    pub signature_data: Option<ByteVec>,
}

impl Transaction {
    pub fn new(
        stamp: Stamp,
        sequence: i32,
        fee: i64,
        memo: String,
        payload: Box<dyn payload::Payload>,
        public_key: Option<PublicKey>,
        signature: Option<Signature>,
    ) -> Result<Self> {
        Ok(Transaction {
            stamp,
            sequence,
            fee,
            memo,
            payload,
            public_key,
            signature,
        })
    }
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        let raw: RawTransaction = minicbor::decode(data)?;
        let payload = Box::new(match raw.payload_type {
            payload::Type::Send => {
                minicbor::decode::<payload::send::SendPayload>(raw.payload_data.as_ref())?
            }
            _ => minicbor::decode::<payload::send::SendPayload>(raw.payload_data.as_ref())?,
        });

        let signature = match raw.signature_data {
            Some(data) => Some(Signature::from_bytes(KeyPairType::KeyPairBLS, &data)?),
            None => None,
        };
        let public_key = match raw.public_key_data {
            Some(data) => Some(PublicKey::from_bytes(KeyPairType::KeyPairBLS, &data)?),
            None => None,
        };

        Self::new(
            raw.stamp,
            raw.sequence,
            raw.fee,
            raw.memo,
            payload,
            public_key,
            signature,
        )
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let payload_data = ByteVec::from(self.payload.to_bytes()?);
        let public_key_data = match self.public_key.as_ref() {
            Some(pk) => Some(ByteVec::from(pk.to_bytes())),
            None => None,
        };
        let signature_data = match self.signature.as_ref() {
            Some(sig) => Some(ByteVec::from(sig.to_bytes())),
            None => None,
        };

        let raw = RawTransaction {
            version: 1,
            stamp: self.stamp.clone(),
            sequence: self.sequence,
            fee: self.fee,
            memo: self.memo.clone(),
            payload_type: self.payload.payload_type(),
            payload_data,
            public_key_data,
            signature_data,
        };
        Ok(minicbor::to_vec(raw)?)
    }

    fn sign_bytes(&self) -> Result<Vec<u8>> {
        let payload_data = ByteVec::from(self.payload.to_bytes()?);
        let raw = RawTransaction {
            version: 1,
            stamp: self.stamp.clone(),
            sequence: self.sequence,
            fee: self.fee,
            memo: self.memo.clone(),
            payload_type: self.payload.payload_type(),
            payload_data,
            public_key_data: None,
            signature_data: None,
        };
        Ok(minicbor::to_vec(raw)?)
    }

    pub fn check_signature(&self) -> bool {
        self.public_key.as_ref().unwrap().verify(
            &self.signature.as_ref().unwrap(),
            &self.sign_bytes().unwrap(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decoding() {
        let buf1 = hex::decode(
            "a901010244e4f59ccd03186e041903e80501065833a3015501d75c059a4157d78f9b86741164037392de0fa53102550194f782f332649a4234b79216277e0b1594836313031903e8076c746573742073656e642d7478145860a4de42541ddeebfa6c4c8f008d2a64e6a2c8069096a5ad2fd807089a2f3ca8b71554365a01a2a3d5eee73f814b2aaeee0a49496e9222bc5cb4e9ffec219b4dca5091844ac1752286a524ca89928187ea60d0bdd6f10047d06f204bac5c215967155830b1c1b312df0ac1877c8daeb35eaf53c5008fb1de9654c698bab851b73d8730204c5c93c13c7d5d6b29ee439d1bdb7118",
        ).unwrap();

        let buf2 = hex::decode(
            "a701010244e4f59ccd03186e041903e80501065833a3015501d75c059a4157d78f9b86741164037392de0fa53102550194f782f332649a4234b79216277e0b1594836313031903e8076c746573742073656e642d7478",
        ).unwrap();
        let trx = Transaction::from_bytes(buf1.as_slice()).unwrap();
        assert_eq!(buf1, trx.to_bytes().unwrap());
        assert_eq!(buf2, trx.sign_bytes().unwrap());
    }
}
