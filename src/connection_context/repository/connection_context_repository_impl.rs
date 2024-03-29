use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::net::TcpStream;

use tokio::sync::{Mutex as AsyncMutex, Mutex};
use crate::client_socket_accept::entity::client_socket::ClientSocket;
use crate::client_socket_accept::repository::client_socket_accept_repository_impl::ReceiverTransmitterChannel;

use crate::connection_context::repository::connection_context_repository::ConnectionContextRepository;

#[derive(Clone)]
pub struct ConnectionContextRepositoryImpl {
    connection_context_map: Arc<AsyncMutex<HashMap<i32, Arc<AsyncMutex<ClientSocket>>>>>,
}

impl ConnectionContextRepositoryImpl {
    pub fn new() -> Self {
        ConnectionContextRepositoryImpl {
            connection_context_map: Arc::new(AsyncMutex::new(HashMap::new())),
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<ConnectionContextRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<ConnectionContextRepositoryImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        ConnectionContextRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
    pub fn connection_context_map(&self) -> &Arc<AsyncMutex<HashMap<i32, Arc<AsyncMutex<ClientSocket>>>>> {
        &self.connection_context_map
    }
}

#[async_trait]
impl ConnectionContextRepository for ConnectionContextRepositoryImpl {
    async fn add_connection_context(
        &mut self,
        account_unique_id: i32,
        // socket_stream: Arc<AsyncMutex<TcpStream>>,
        // each_receiver_transmitter_channel: &Arc<ReceiverTransmitterChannel>,
        client_socket: Arc<Mutex<ClientSocket>>,
    ) -> bool {
        println!("ConnectionContextRepositoryImpl: add_connection_context()");

        let mut map = self.connection_context_map();
        let mut map_guard = map.lock().await;
        map_guard.insert(account_unique_id, client_socket);
        println!("Connection context map after adding: {:?}", *map_guard);

        true
    }
}

#[cfg(test)]
mod tests {
    use std::net::SocketAddr;
    use std::time::Duration;
    use tokio::net::TcpListener;
    use tokio::sync::Mutex;
    use super::*;
    use tokio::test;
    use tokio::time::sleep;

    async fn fake_server(listener: Arc<Mutex<TcpListener>>, repository: ConnectionContextRepositoryImpl) {
        while let Ok((socket, _)) = listener.lock().await.accept().await {
            let account_unique_id = 1;
            // let socket_stream = Arc::new(AsyncMutex::new(socket));
            // let socket_stream_guard = socket_stream.lock().await;
            // let socket = &*socket_stream_guard;
            let client_socket = Arc::new(
                AsyncMutex::new(
                    ClientSocket::new(
                        "fake".parse().unwrap(),
                        socket,
                        Arc::new(ReceiverTransmitterChannel::new(1)))));
            let mut repository_clone = repository.clone();
            repository_clone.add_connection_context(account_unique_id, client_socket, ).await;
            println!("Fake server: New connection added for account_unique_id {}", account_unique_id);
        }
    }

    #[tokio::test]
    async fn test_add_connection_context() {
        let listener = Arc::new(Mutex::new(TcpListener::bind("127.0.0.1:0").await.unwrap()));
        let addr: SocketAddr = listener.lock().await.local_addr().unwrap();

        let mut repository = ConnectionContextRepositoryImpl::get_instance();
        let mut repository_guard = repository.lock().await;

        tokio::spawn(fake_server(listener.clone(), repository_guard.clone()));

        let stream = TcpStream::connect(addr).await.unwrap();

        let account_unique_id = 2;
        // let socket_stream = Arc::new(AsyncMutex::new(stream));

        let client_socket = Arc::new(
            AsyncMutex::new(
                ClientSocket::new(
                    "fake".parse().unwrap(),
                    stream,
                    Arc::new(ReceiverTransmitterChannel::new(1)))));

        let result = repository_guard.add_connection_context(account_unique_id, client_socket.clone(), ).await;

        assert!(result);
        assert_eq!(repository_guard.connection_context_map().lock().await.len(), 1);
        assert!(matches!(
            repository_guard.connection_context_map().lock().await.get(&account_unique_id),
            Some(value) if Arc::ptr_eq(value, &client_socket)
        ));

        sleep(Duration::from_millis(100)).await;

        println!("Connection context map: {:?}", repository_guard.connection_context_map().lock().await);
    }
}
