pub trait PublicKey {
    type Signature: crate::signature::Signature;

    fn verify(&self, sig: &Self::Signature, msg: &[u8]) -> bool;
    fn to_bytes(&self) -> Vec<u8>;
}
