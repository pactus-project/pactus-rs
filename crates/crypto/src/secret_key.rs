
pub trait SecretKey {
   type PublicKey: crate::public_key::PublicKey;
   type Signature: crate::signature::Signature;

    fn public_key(&self) -> Self::PublicKey;
    fn sign(&self, msg: &[u8]) ->Self::Signature;
    fn to_bytes(&self) -> Vec<u8>;
}
