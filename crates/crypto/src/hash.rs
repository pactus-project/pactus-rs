use minicbor::{Decode, Encode};

const HASH32_SIZE: usize = 12;

#[derive(Encode, Decode)]
pub struct Hash32 {
    #[n(1)]
    data: [u8; HASH32_SIZE],
}
