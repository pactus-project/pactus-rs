pub mod public_key;
pub mod secret_key;
pub mod signature;

macro_rules! impl_common {
    () => {
        pub fn from_string(hex: &str) -> Result<Self> {
            let data = hex::decode(hex)?;
            Self::from_bytes(&data)
        }

        pub fn to_string(&self) -> String {
            hex::encode(self.to_bytes())
        }
    };
}

pub(super) use impl_common;
