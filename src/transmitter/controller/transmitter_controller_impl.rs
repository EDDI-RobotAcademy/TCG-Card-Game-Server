use std::sync::Arc;
use async_trait::async_trait;
use ipc_channel::ipc::IpcReceiver;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;
use crate::domain_initializer::initializer::{AcceptorTransmitterChannel, ReceiverTransmitterLegacyChannel};
use crate::receiver::controller::server_receiver_controller::ServerReceiverController;
use crate::receiver::controller::server_receiver_controller_impl::ServerReceiverControllerImpl;
use crate::receiver::service::server_receiver_service_impl::ServerReceiverServiceImpl;
use crate::response_generator::response_type::ResponseType;
use crate::transmitter::controller::transmitter_controller::TransmitterController;
use crate::transmitter::service::transmitter_service::TransmitterService;

use crate::transmitter::service::transmitter_service_impl::TransmitterServiceImpl;

pub struct TransmitterControllerImpl {
    transmitter_service: Arc<AsyncMutex<TransmitterServiceImpl>>,
}

impl TransmitterControllerImpl {
    pub fn new(transmitter_service:
               Arc<AsyncMutex<TransmitterServiceImpl>>) -> Self {

        TransmitterControllerImpl {
            transmitter_service,
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<TransmitterControllerImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<TransmitterControllerImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        TransmitterControllerImpl::new(
                            TransmitterServiceImpl::get_instance())));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl TransmitterController for TransmitterControllerImpl {
    async fn transmit_to_client(&mut self) {
        println!("TransmitterControllerImpl: transmit_to_client()");

        let mut transmitter_gaurd = self.transmitter_service.lock().await;
        transmitter_gaurd.transmit_to_client().await;
    }

    async fn inject_acceptor_transmitter_channel(&mut self, acceptor_transmitter_channel_arc: Arc<AcceptorTransmitterChannel>) {
        println!("TransmitterControllerImpl: inject_acceptor_transmitter_channel()");

        let mut transmitter_gaurd = self.transmitter_service.lock().await;
        transmitter_gaurd.inject_acceptor_transmitter_channel(acceptor_transmitter_channel_arc).await;
    }

    async fn inject_receiver_transmitter_channel(&mut self, receiver_transmitter_channel_arc: Arc<ReceiverTransmitterLegacyChannel>) {
        println!("TransmitterControllerImpl: inject_receiver_transmitter_channel()");

        let mut transmitter_gaurd = self.transmitter_service.lock().await;
        transmitter_gaurd.inject_receiver_transmitter_channel(receiver_transmitter_channel_arc).await;
    }
}
