use std::sync::Arc;
use async_trait::async_trait;
use ipc_channel::ipc::IpcReceiver;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;
use crate::domain_initializer::initializer::AcceptorTransmitterChannel;
use crate::response_generator::response_type::ResponseType;

use crate::transmitter::entity::transmit_data::TransmitData;
use crate::transmitter::repository::transmitter_repository::TransmitterRepository;

pub struct TransmitterRepositoryImpl {
    transmit_data: TransmitData,
    acceptor_transmitter_channel_arc: Option<Arc<AcceptorTransmitterChannel>>,
    receiver_transmitter_tx: Option<IpcReceiver<ResponseType>>
}

impl TransmitterRepositoryImpl {
    pub fn new() -> Self {
        TransmitterRepositoryImpl {
            transmit_data: TransmitData::new(),
            acceptor_transmitter_channel_arc: None,
            receiver_transmitter_tx: None
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

        let acceptor_channel = self.acceptor_transmitter_channel_arc.clone();

        let join_handle = tokio::task::spawn(async move {
            while let Some(stream_arc) = acceptor_channel.clone().expect("Need to inject channel").receive().await {
                println!("Transmitter transmit() loop");

                // Clone channels for use in the spawned task
                let acceptor_transmitter_channel_arc = self.acceptor_transmitter_channel_arc.clone();
                let receiver_transmitter_tx = self.receiver_transmitter_tx.clone().map(|rx| rx.clone());

                // Spawn a new async task to handle the socket
                tokio::spawn(async move {
                    // Access shared data with a Mutex
                    let acceptor_transmitter_channel = acceptor_transmitter_channel_arc.lock().await;

                    // Your logic to handle the socket goes here
                    // You can use acceptor_transmitter_channel and receiver_transmitter_tx

                    // For example, waiting for receiver to be ready
                    while receiver_transmitter_tx.is_none() {
                        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    }

                    // Now you can use receiver_transmitter_tx to send responses

                });
            }

            println!("Transmitter transmit() end");
        });

        join_handle.await.expect("Failed to await spawned task");

        println!("TransmitterRepositoryImpl: transmit() end");
    }

    async fn inject_accept_transmitter_channel(&mut self, acceptor_transmitter_channel_arc: Arc<AcceptorTransmitterChannel>) {
        println!("TransmitterRepository: inject_accept_transmitter_channel()");

        self.acceptor_transmitter_channel_arc = Option::from(acceptor_transmitter_channel_arc);
    }

    async fn inject_receiver_transmitter_channel(&mut self, receiver_transmitter_tx: IpcReceiver<ResponseType>) {
        println!("TransmitterRepository: inject_receiver_transmitter_channel()");

        self.receiver_transmitter_tx = Option::from(receiver_transmitter_tx);
    }
}

