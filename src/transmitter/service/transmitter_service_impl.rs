use std::sync::Arc;
use async_trait::async_trait;
use ipc_channel::ipc::IpcReceiver;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;
use crate::domain_initializer::initializer::AcceptorTransmitterChannel;
use crate::receiver::repository::server_receiver_repository_impl::ServerReceiverRepositoryImpl;
use crate::receiver::service::server_receiver_service::ServerReceiverService;
use crate::receiver::service::server_receiver_service_impl::ServerReceiverServiceImpl;
use crate::response_generator::response_type::ResponseType;
use crate::transmitter::repository::transmitter_repository::TransmitterRepository;

use crate::transmitter::repository::transmitter_repository_impl::TransmitterRepositoryImpl;
use crate::transmitter::service::transmitter_service::TransmitterService;


pub struct TransmitterServiceImpl {
    transmitter_repository: Arc<AsyncMutex<TransmitterRepositoryImpl>>,
}

impl TransmitterServiceImpl {
    pub fn new(transmitter_repository:
               Arc<AsyncMutex<TransmitterRepositoryImpl>>) -> Self {

        TransmitterServiceImpl {
            transmitter_repository,
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<TransmitterServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<TransmitterServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        TransmitterServiceImpl::new(
                            TransmitterRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl TransmitterService for TransmitterServiceImpl {
    async fn transmit_to_client(&mut self) {
        println!("TransmitterServiceImpl: transmit_to_client()");

        let mut transmitter_repository_gaurd = self.transmitter_repository.lock().await;
        transmitter_repository_gaurd.transmit().await;
    }

    async fn inject_acceptor_transmitter_channel(&mut self, acceptor_transmitter_channel_arc: Arc<AcceptorTransmitterChannel>) {
        println!("TransmitterServiceImpl: inject_accept_transmitter_channel()");

        let mut transmitter_repository_gaurd = self.transmitter_repository.lock().await;
        transmitter_repository_gaurd.inject_accept_transmitter_channel(acceptor_transmitter_channel_arc).await;
    }

    async fn inject_receiver_transmitter_channel(&mut self, receiver_transmitter_tx: IpcReceiver<ResponseType>) {
        println!("TransmitterServiceImpl: inject_receiver_transmitter_channel()");

        let mut transmitter_repository_gaurd = self.transmitter_repository.lock().await;
        transmitter_repository_gaurd.inject_receiver_transmitter_channel(receiver_transmitter_tx).await;
    }
}
