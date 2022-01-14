pub trait Signature {
    fn to_bytes(&self) -> Vec<u8>;
}
