use async_trait::async_trait;

pub mod config;
pub mod error;
pub mod network;
pub mod sync;

#[async_trait]
pub trait Service {
    async fn start(self);
}


pub fn agent() -> String {
    format!("zarb-rs/{}", env!("CARGO_PKG_VERSION"))
}