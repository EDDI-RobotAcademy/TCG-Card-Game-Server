use tokio::net::TcpListener;
use std::sync::Arc;
use tokio::sync::Mutex as AsyncMutex;
use async_trait::async_trait;
use lazy_static::lazy_static;
use crate::server_socket::repository::server_socket_repository::ServerSocketRepository;

#[derive(Debug)]
pub struct ServerSocketRepositoryImpl {
    listener: Option<Arc<AsyncMutex<TcpListener>>>,
}

impl Clone for ServerSocketRepositoryImpl {
    fn clone(&self) -> Self {
        ServerSocketRepositoryImpl {
            listener: None,
        }
    }
}

impl Eq for ServerSocketRepositoryImpl {}

impl PartialEq for ServerSocketRepositoryImpl {
    fn eq(&self, other: &Self) -> bool {
        self.listener.is_none() && other.listener.is_none()
    }
}

impl ServerSocketRepositoryImpl {
    pub fn new() -> Self {
        ServerSocketRepositoryImpl {
            listener: None,
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<ServerSocketRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<ServerSocketRepositoryImpl>> =
                Arc::new(AsyncMutex::new(ServerSocketRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl ServerSocketRepository for ServerSocketRepositoryImpl {
    async fn bind_socket(&mut self, address: &str) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(address)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        let listener = Arc::new(AsyncMutex::new(listener));

        self.listener = Some(listener.clone());
        if let Some(listener) = &self.listener {
            println!("Listener bound successfully: {}", listener.lock().await.local_addr().unwrap());
        }
        Ok(())
    }

    async fn get_listener(&self) -> Option<Arc<AsyncMutex<TcpListener>>> {
        self.listener.clone()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use super::*;

    #[tokio::test]
    async fn test_bind_socket() {
        let repository_mutex = ServerSocketRepositoryImpl::get_instance();
        let mut repository_guard = repository_mutex.lock().await;
        let address = "127.0.0.1:37373";

        match repository_guard.bind_socket(address).await {
            Ok(()) => {
                if let Some(listener_option) = repository_guard.get_listener().await {
                    let listener_mutex = listener_option.lock().await;
                    let bound_address = listener_mutex.local_addr().unwrap();
                    assert_eq!(bound_address.to_string(), address);
                } else {
                    panic!("Listener not found after successful bind");
                }
            }
            Err(err) => {
                panic!("Error binding socket: {:?}", err);
            }
        }
    }

    #[tokio::test]
    async fn test_get_listener_bound() {
        let repository_mutex = ServerSocketRepositoryImpl::get_instance();
        let mut repository_guard = repository_mutex.lock().await;
        let address = "127.0.0.1:9787";

        match repository_guard.bind_socket(address).await {
            Ok(()) => {
                let listener_option = repository_guard.get_listener().await;
                assert!(listener_option.is_some());
            }
            Err(err) => {
                panic!("Error binding socket: {:?}", err);
            }
        }
    }

    #[tokio::test]
    async fn test_get_instance() {
        let instance1 = ServerSocketRepositoryImpl::get_instance();
        let instance2 = ServerSocketRepositoryImpl::get_instance();

        assert!(Arc::ptr_eq(&instance1, &instance2));
    }
}

