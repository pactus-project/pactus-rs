use async_trait::async_trait;

pub mod config;
pub mod error;
pub mod network;
pub mod sync;

#[async_trait]
pub trait Service {
    async fn start(self);
}
