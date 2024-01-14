use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex as AsyncMutex;
use lazy_static::lazy_static;
use tokio::net::TcpListener;
use crate::server_socket::repository::server_socket_repository::ServerSocketRepository;
use crate::server_socket::repository::server_socket_repository_impl::ServerSocketRepositoryImpl;
use crate::server_socket::service::server_socket_service::ServerSocketService;

#[derive(Clone)]
pub struct ServerSocketServiceImpl {
    repository: Arc<AsyncMutex<ServerSocketRepositoryImpl>>,
}

impl ServerSocketServiceImpl {
    pub fn new(repository: Arc<AsyncMutex<ServerSocketRepositoryImpl>>) -> Self {
        ServerSocketServiceImpl { repository }
    }

    pub fn get_instance() -> Arc<AsyncMutex<ServerSocketServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<ServerSocketServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        ServerSocketServiceImpl::new(
                            ServerSocketRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl ServerSocketService for ServerSocketServiceImpl {
    async fn server_socket_bind(&mut self, address: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut repository_guard = self.repository.lock().await;
        repository_guard.bind_socket(address).await
    }

    async fn get_listener(&self) -> Option<Arc<AsyncMutex<TcpListener>>> {
        let repository_guard = self.repository.lock().await;
        repository_guard.get_listener().await
    }
}

impl AsRef<ServerSocketRepositoryImpl> for ServerSocketRepositoryImpl {
    fn as_ref(&self) -> &ServerSocketRepositoryImpl {
        self
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bind_service() {
        let service = ServerSocketServiceImpl::get_instance();
        let address = "127.0.0.1:27373";

        let mut service_guard = service.lock().await;

        assert!(service_guard.server_socket_bind(address).await.is_ok());
    }

    #[tokio::test]
    async fn test_service_repository_bind() {
        let server_socket_service = ServerSocketServiceImpl::get_instance();
        let mut service_guard = server_socket_service.lock().await;

        let expected_address = "127.0.0.1:12817";
        match service_guard.server_socket_bind(expected_address).await {
            Ok(()) => {
                let tcp_listener_option = service_guard.get_listener().await;

                if let Some(tcp_listener) = tcp_listener_option {
                    let tcp_listener_mutex = tcp_listener.lock().await;
                    let local_addr = tcp_listener_mutex.local_addr().expect("Failed to get local address");

                    println!("Bound to address: {}", local_addr);

                    assert_eq!(local_addr.to_string(), expected_address);
                } else {
                    panic!("Listener is None after successful bind");
                }
            }
            Err(err) => {
                panic!("Error binding socket: {:?}", err);
            }
        }
    }
}
