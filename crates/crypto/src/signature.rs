use minicbor::{Decode, Encode};

pub trait Signature<'a>: Decode<'a> + Encode {
    fn to_bytes(&self) -> Vec<u8>;
}
