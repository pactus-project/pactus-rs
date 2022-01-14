use std::fmt::Debug;

use crate::public_key::PublicKey;
use crate::signature::Signature;

pub trait Signatory: Debug {
    fn verify(&self, msg: &[u8]) -> bool;
    fn public_key(&self) -> &dyn PublicKey;
    fn signature(&self) -> &dyn Signature;
}
