#[async_trait::async_trait]
pub trait Reporter: Send + Sync {
    async fn start(&self);
}
