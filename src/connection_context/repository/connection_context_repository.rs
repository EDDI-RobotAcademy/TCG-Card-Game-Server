use std::sync::Arc;
use async_trait::async_trait;
use tokio::net::TcpStream;
use tokio::sync::Mutex as AsyncMutex;

#[async_trait]
pub trait ConnectionContextRepository {
    async fn add_connection_context(
        &mut self,
        account_unique_id: i32,
        socket_stream: Arc<AsyncMutex<TcpStream>>,
    ) -> bool;
}