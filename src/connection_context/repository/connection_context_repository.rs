use std::sync::Arc;
use async_trait::async_trait;
use tokio::net::TcpStream;
use tokio::sync::{Mutex as AsyncMutex, Mutex};
use crate::client_socket_accept::entity::client_socket::ClientSocket;
use crate::client_socket_accept::repository::client_socket_accept_repository_impl::ReceiverTransmitterChannel;

#[async_trait]
pub trait ConnectionContextRepository {
    async fn add_connection_context(
        &mut self,
        account_unique_id: i32,
        // socket_stream: Arc<AsyncMutex<TcpStream>>,
        // each_receiver_transmitter_channel: &Arc<ReceiverTransmitterChannel>,
        client_socket: Arc<Mutex<ClientSocket>>,
    ) -> bool;
}