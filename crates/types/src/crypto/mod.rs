pub mod bls;
pub mod public_key;
pub mod secret_key;
pub mod signature;
pub mod signer;

macro_rules! impl_common {
    () => {
        pub fn from_string(key_type: KeyPairType,hex: &str) -> Result<Self> {
            let data = hex::decode(hex)?;
            Self::from_bytes(key_type, &data)
        }

        pub fn to_string(&self) -> String {
            hex::encode(self.to_bytes())
        }
    };
}

pub(super) use impl_common;

pub enum KeyPairType {
    KeyPairBLS,
}
