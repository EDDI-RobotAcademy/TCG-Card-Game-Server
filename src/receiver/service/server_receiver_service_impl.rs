use std::sync::Arc;
use async_trait::async_trait;
use ipc_channel::ipc::IpcSender;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;
use crate::receiver::repository::server_receiver_repository::ServerReceiverRepository;
use crate::receiver::repository::server_receiver_repository_impl::ServerReceiverRepositoryImpl;
use crate::receiver::service::server_receiver_service::ServerReceiverService;
use crate::domain_initializer::initializer::{AcceptorReceiverChannel, AcceptorTransmitterChannel};
use crate::response_generator::response_type::ResponseType;

pub struct ServerReceiverServiceImpl {
    server_receiver_repository: Arc<AsyncMutex<ServerReceiverRepositoryImpl>>,
}

impl ServerReceiverServiceImpl {
    pub fn new(server_receiver_repository:
               Arc<AsyncMutex<ServerReceiverRepositoryImpl>>) -> Self {

        ServerReceiverServiceImpl {
            server_receiver_repository,
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<ServerReceiverServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<ServerReceiverServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        ServerReceiverServiceImpl::new(
                            ServerReceiverRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl ServerReceiverService for ServerReceiverServiceImpl {
    async fn client_receive(&mut self) {
        println!("ServerReceiverServiceImpl: receive()");

        let server_receiver_repository_mutex = &self.server_receiver_repository;
        let mut server_receiver_repository_guard = server_receiver_repository_mutex.lock().await;
        server_receiver_repository_guard.receive().await;
    }

    async fn inject_acceptor_receiver_channel(&mut self, acceptor_receiver_channel_arc: Arc<AcceptorReceiverChannel>) {
        println!("ServerReceiverServiceImpl: inject_acceptor_receiver_channel()");
        let mut server_receiver_repository_gaurd = self.server_receiver_repository.lock().await;
        server_receiver_repository_gaurd.inject_acceptor_receiver_channel(acceptor_receiver_channel_arc).await;
    }

    async fn inject_receiver_transmitter_channel(&mut self, receiver_transmitter_tx: IpcSender<ResponseType>) {
        println!("ServerReceiverServiceImpl: inject_receiver_transmitter_channel()");
        let mut server_receiver_repository_gaurd = self.server_receiver_repository.lock().await;
        server_receiver_repository_gaurd.inject_receiver_transmitter_channel(receiver_transmitter_tx).await;
    }
}
