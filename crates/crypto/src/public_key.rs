use minicbor::{Decode, Encode};

pub trait PublicKey<'a>: Decode<'a> + Encode {
    type Signature: crate::signature::Signature<'a>;

    fn verify(&self, sig: Self::Signature, msg: &[u8]) -> bool;
    fn to_bytes(&self) -> Vec<u8>;
}
