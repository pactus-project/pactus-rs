use std::fmt::Debug;

pub trait Signature: Debug {
    fn to_bytes(&self) -> Vec<u8>;
}
