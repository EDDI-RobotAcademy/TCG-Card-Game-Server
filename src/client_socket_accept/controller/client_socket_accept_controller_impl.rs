use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;
use crate::client_socket_accept::controller::client_socket_accept_controller::ClientSocketAcceptController;
use crate::client_socket_accept::service::client_socket_accept_service::ClientSocketAcceptService;
use crate::client_socket_accept::service::client_socket_accept_service_impl::ClientSocketAcceptServiceImpl;

use crate::domain_initializer::initializer::AcceptorReceiverChannel;

#[derive(Clone)]
pub struct ClientSocketAcceptControllerImpl {
    service: Arc<AsyncMutex<ClientSocketAcceptServiceImpl>>,
}

impl ClientSocketAcceptControllerImpl {
    pub fn new(service: Arc<AsyncMutex<ClientSocketAcceptServiceImpl>>) -> Self {
        ClientSocketAcceptControllerImpl { service }
    }

    pub fn get_instance() -> Arc<AsyncMutex<ClientSocketAcceptControllerImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<ClientSocketAcceptControllerImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        ClientSocketAcceptControllerImpl::new(
                            ClientSocketAcceptServiceImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl ClientSocketAcceptController for ClientSocketAcceptControllerImpl {
    async fn accept_client(&self) {
        println!("Client Socket Accept Controller: accept()");
        let client_socket_accept_guard = self.service.lock().await;
        client_socket_accept_guard.accept_client().await;
    }

    async fn inject_accept_channel(&self, acceptor_receiver_channel_arc: Arc<AcceptorReceiverChannel>) {
        let client_socket_accept_service_guard = self.service.lock().await;
        client_socket_accept_service_guard.inject_accept_channel(acceptor_receiver_channel_arc).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::Duration;
    use crate::server_socket::repository::server_socket_repository::ServerSocketRepository;
    use crate::server_socket::repository::server_socket_repository_impl::ServerSocketRepositoryImpl;


    #[tokio::test]
    async fn test_controller_singleton_behavior() {
        let acceptor_channel = AcceptorReceiverChannel::new(1);
        let acceptor_channel_arc = Arc::new(acceptor_channel.clone());

        let controller1 = ClientSocketAcceptControllerImpl::get_instance();
        let controller2 = ClientSocketAcceptControllerImpl::get_instance();

        assert!(Arc::ptr_eq(&controller1, &controller2));
    }

    #[tokio::test]
    async fn test_controller_accept_client() {
        let acceptor_receiver_channel = AcceptorReceiverChannel::new(1);
        let acceptor_receiver_channel_arc = Arc::new(acceptor_receiver_channel.clone());

        let client_socket_accept_controller_mutex = ClientSocketAcceptControllerImpl::get_instance();
        let server_socket_service_mutex = ServerSocketRepositoryImpl::get_instance();

        tokio::spawn(async move {
            let client_socket_accept_controlelr_gaurd = client_socket_accept_controller_mutex.lock().await;

            let mut server_socket_service_guard = server_socket_service_mutex.lock().await;
            let address = "127.0.0.1:7373";
            server_socket_service_guard.bind_socket(address).await.expect("TODO: bind failed");
            
            let listener_option = server_socket_service_guard.get_listener().await;
            assert!(listener_option.is_some());

            drop(server_socket_service_guard);

            client_socket_accept_controlelr_gaurd.accept_client().await;
        });

        tokio::time::sleep(Duration::from_secs(5)).await;
    }

    #[tokio::test]
    async fn test_get_client_socket_accept_controller() {
        let acceptor_receiver_channel = AcceptorReceiverChannel::new(1);
        let acceptor_receiver_channel_arc = Arc::new(acceptor_receiver_channel.clone());

        let controller1_mutex = ClientSocketAcceptControllerImpl::get_instance();
        let controller2_mutex = ClientSocketAcceptControllerImpl::get_instance();

        assert!(Arc::ptr_eq(&controller1_mutex, &controller2_mutex));
    }
}