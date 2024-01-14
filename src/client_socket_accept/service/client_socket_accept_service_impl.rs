use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;
use crate::client_socket_accept::repository::client_socket_accept_repository::ClientSocketAcceptRepository;
use crate::client_socket_accept::repository::client_socket_accept_repository_impl::ClientSocketAcceptRepositoryImpl;
use crate::client_socket_accept::service::client_socket_accept_service::ClientSocketAcceptService;
use crate::server_socket::repository::server_socket_repository::ServerSocketRepository;
use crate::server_socket::repository::server_socket_repository_impl::ServerSocketRepositoryImpl;
use crate::domain_initializer::initializer::AcceptorReceiverChannel;

#[derive(Clone)]
pub struct ClientSocketAcceptServiceImpl {
    client_socket_accept_repository: Arc<AsyncMutex<ClientSocketAcceptRepositoryImpl>>,
    server_socket_repository: Arc<AsyncMutex<ServerSocketRepositoryImpl>>
}

impl ClientSocketAcceptServiceImpl {
    pub fn new(client_socket_accept_repository: Arc<AsyncMutex<ClientSocketAcceptRepositoryImpl>>,
               server_socket_repository: Arc<AsyncMutex<ServerSocketRepositoryImpl>>) -> Self {

        ClientSocketAcceptServiceImpl {
            client_socket_accept_repository,
            server_socket_repository
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<ClientSocketAcceptServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<ClientSocketAcceptServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        ClientSocketAcceptServiceImpl::new(
                            ClientSocketAcceptRepositoryImpl::get_instance(),
                            ServerSocketRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl ClientSocketAcceptService for ClientSocketAcceptServiceImpl {

    async fn accept_client(&self) {
        println!("Client Socket Accept Service: accept()");

        let server_socket_repository_guard = self.server_socket_repository.lock().await;
        println!("get server_socket_repository lock");

        if let Some(listener) = server_socket_repository_guard.get_listener().await {
            println!("get client_socket_accept_repository lock");
            let mut client_socket_accept_guard = self.client_socket_accept_repository.lock().await;
            let listener_guard = listener.lock().await;
            client_socket_accept_guard.accept_client(&*listener_guard).await;
        } else {
            eprintln!("Listener not available for accepting clients.");
        }
    }

    async fn inject_accept_channel(&self, acceptor_receiver_channel_arc: Arc<AcceptorReceiverChannel>) {
        let mut client_socket_accept_service_guard = self.client_socket_accept_repository.lock().await;
        client_socket_accept_service_guard.inject_accept_channel(acceptor_receiver_channel_arc).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::Duration;

    #[tokio::test]
    async fn test_singleton_behavior() {
        let instance1 = ClientSocketAcceptServiceImpl::get_instance();
        let instance2 = ClientSocketAcceptServiceImpl::get_instance();

        assert!(Arc::ptr_eq(&instance1, &instance2));
    }

    #[tokio::test]
    async fn test_accept_client() {
        let client_socket_accept_repository = ClientSocketAcceptRepositoryImpl::get_instance();

        let server_socket_repository = ServerSocketRepositoryImpl::get_instance();

        tokio::spawn(async move {
            let mut server_socket_guard = server_socket_repository.lock().await;

            let address = "127.0.0.1:7738";
            server_socket_guard.bind_socket(address).await.expect("TODO: panic message");

            let listener_option = server_socket_guard.get_listener().await;
            let listener_option_bind = listener_option.unwrap();
            let listener = listener_option_bind.lock().await;

            let mut client_socket_accept_guard = client_socket_accept_repository.lock().await;

            client_socket_accept_guard.accept_client(&*listener).await;
        });

        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}
