use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::net::TcpStream;

use tokio::sync::Mutex as AsyncMutex;

use crate::connection_context::repository::connection_context_repository::ConnectionContextRepository;

#[derive(Clone)]
pub struct ConnectionContextRepositoryImpl {
    connection_context_map: Arc<AsyncMutex<HashMap<i32, Arc<AsyncMutex<TcpStream>>>>>,
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
    pub fn connection_context_map(&self) -> &Arc<AsyncMutex<HashMap<i32, Arc<AsyncMutex<TcpStream>>>>> {
        &self.connection_context_map
    }
}

#[async_trait]
impl ConnectionContextRepository for ConnectionContextRepositoryImpl {
    async fn add_connection_context(
        &mut self,
        account_unique_id: i32,
        socket_stream: Arc<AsyncMutex<TcpStream>>,
    ) -> bool {
        println!("ConnectionContextRepositoryImpl: add_connection_context()");

        let mut map = self.connection_context_map();
        let mut map_guard = map.lock().await;
        map_guard.insert(account_unique_id, socket_stream);
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
            let socket_stream = Arc::new(AsyncMutex::new(socket));
            let mut repository_clone = repository.clone();
            repository_clone.add_connection_context(account_unique_id, socket_stream.clone()).await;
            println!("Fake server: New connection added for account_unique_id {}", account_unique_id);
        }
    }

    #[tokio::test]
    async fn test_add_connection_context() {
        let listener = Arc::new(Mutex::new(TcpListener::bind("127.0.0.1:0").await.unwrap()));
        let addr: SocketAddr = listener.lock().await.local_addr().unwrap();

        // ConnectionContextRepositoryImpl instance creation
        let mut repository = ConnectionContextRepositoryImpl::get_instance();
        let mut repository_guard = repository.lock().await;

        // Spawn a fake server in a separate task
        tokio::spawn(fake_server(listener.clone(), repository_guard.clone()));

        // Connect to the fake server
        let stream = TcpStream::connect(addr).await.unwrap();

        // Perform the actual test using the connected stream
        let account_unique_id = 2;
        let socket_stream = Arc::new(AsyncMutex::new(stream));
        let result = repository_guard.add_connection_context(account_unique_id, socket_stream.clone()).await;

        assert!(result);
        assert_eq!(repository_guard.connection_context_map().lock().await.len(), 1);
        assert!(matches!(
            repository_guard.connection_context_map().lock().await.get(&account_unique_id),
            Some(value) if Arc::ptr_eq(value, &socket_stream)
        ));

        // Sleep for a short time to allow the fake server to finish processing
        sleep(Duration::from_millis(100)).await;

        println!("Connection context map: {:?}", repository_guard.connection_context_map().lock().await);
    }
}
