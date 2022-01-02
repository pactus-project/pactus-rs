
pub trait SecretKey {
   type PublicKey: crate::public_key::PublicKey;

    fn public_key(&self) -> Self::PublicKey;
}
