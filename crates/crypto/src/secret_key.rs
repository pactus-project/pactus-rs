use minicbor::{Decode, Encode};

pub trait SecretKey<'a>: Decode<'a> + Encode {
   type PublicKey: crate::public_key::PublicKey<'a>;
   type Signature: crate::signature::Signature<'a>;

    fn public_key(&self) -> Self::PublicKey;
    fn sign(&self, msg: &[u8]) ->Self::Signature;
    fn to_bytes(&self) -> Vec<u8>;
}
