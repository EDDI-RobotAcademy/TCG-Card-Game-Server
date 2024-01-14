use std::sync::Arc;
use async_trait::async_trait;
use tokio::net::TcpListener;
use tokio::sync::Mutex as AsyncMutex;

#[async_trait]
pub trait ServerSocketService {
    async fn server_socket_bind(&mut self, address: &str) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_listener(&self) -> Option<Arc<AsyncMutex<TcpListener>>>;
}