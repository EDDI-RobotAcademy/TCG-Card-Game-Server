use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex as AsyncMutex;
use async_trait::async_trait;

#[async_trait]
pub trait ServerSocketRepository {
    async fn bind_socket(&mut self, address: &str) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_listener(&self) -> Option<Arc<AsyncMutex<TcpListener>>>;
}