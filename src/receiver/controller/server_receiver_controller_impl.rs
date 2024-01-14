use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;
use crate::receiver::controller::server_receiver_controller::ServerReceiverController;
use crate::receiver::service::server_receiver_service::ServerReceiverService;
use crate::receiver::service::server_receiver_service_impl::ServerReceiverServiceImpl;
use crate::domain_initializer::initializer::AcceptorReceiverChannel;

pub struct ServerReceiverControllerImpl {
    server_receiver_service: Arc<AsyncMutex<ServerReceiverServiceImpl>>,
}

impl ServerReceiverControllerImpl {
    pub fn new(server_receiver_service:
               Arc<AsyncMutex<ServerReceiverServiceImpl>>) -> Self {

        ServerReceiverControllerImpl {
            server_receiver_service,
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<ServerReceiverControllerImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<ServerReceiverControllerImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        ServerReceiverControllerImpl::new(
                            ServerReceiverServiceImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl ServerReceiverController for ServerReceiverControllerImpl {
    async fn client_receive(&mut self) {
        println!("Server Receiver Controller: client_receive()");

        let server_receiver_service_mutex = &self.server_receiver_service;
        let mut server_receiver_service_guard = server_receiver_service_mutex.lock().await;
        server_receiver_service_guard.client_receive().await;
    }

    async fn inject_accept_channel(&mut self, acceptor_receiver_channel_arc: Arc<AcceptorReceiverChannel>) {
        let mut server_receiver_service_guard = self.server_receiver_service.lock().await;
        server_receiver_service_guard.inject_accept_channel(acceptor_receiver_channel_arc).await;
    }
}
