use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;
use crate::domain_initializer::initializer::AcceptorTransmitterChannel;

use crate::transmitter::entity::transmit_data::TransmitData;
use crate::transmitter::repository::transmitter_repository::TransmitterRepository;

pub struct TransmitterRepositoryImpl {
    transmit_data: TransmitData,
    acceptor_transmitter_channel_arc: Option<Arc<AcceptorTransmitterChannel>>,
}

impl TransmitterRepositoryImpl {
    pub fn new() -> Self {
        TransmitterRepositoryImpl {
            transmit_data: TransmitData::new(),
            acceptor_transmitter_channel_arc: None
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<TransmitterRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<TransmitterRepositoryImpl>> =
                Arc::new(AsyncMutex::new(TransmitterRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl TransmitterRepository for TransmitterRepositoryImpl {
    async fn transmit(&mut self) {
        println!("TransmitterRepositoryImpl: transmit()");
    }

    async fn inject_accept_transmitter_channel(&mut self, acceptor_transmitter_channel_arc: Arc<AcceptorTransmitterChannel>) {
        println!("TransmitterRepository: inject_accept_transmitter_channel()");

        self.acceptor_transmitter_channel_arc = Option::from(acceptor_transmitter_channel_arc);
    }
}

