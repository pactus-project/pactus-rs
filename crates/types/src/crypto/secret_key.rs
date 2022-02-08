use std::fmt::Debug;

use super::{public_key::PublicKey, signature::Signature};

pub trait SecretKey: Debug {
    fn public_key(&self) -> Box<dyn PublicKey>;
    fn sign(&self, msg: &[u8]) -> Box<dyn Signature>;
    fn to_bytes(&self) -> Vec<u8>;
}
