pub trait SecretKey {
    type PublicKey: super::PublicKey;
    type Signature: super::Signature;

    fn public_key(&self) -> Self::PublicKey;
    fn sign(&self, msg: &[u8]) -> Self::Signature;
    fn to_bytes(&self) -> Vec<u8>;
}
