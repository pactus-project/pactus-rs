pub trait PublicKey {
    fn to_bytes(&self) -> Vec<u8>;
}
