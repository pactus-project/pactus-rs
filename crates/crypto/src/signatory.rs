pub trait Signatory {
    // type Signature: crate::signature::Signature;
    // type PublicKey: crate::public_key::PublicKey;

    fn verify(&self, msg: &[u8]) -> bool;
}
