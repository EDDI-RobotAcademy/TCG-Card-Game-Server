use std::sync::Arc;
use std::time::Duration;
use async_trait::async_trait;
use ipc_channel::ipc::IpcReceiver;
use lazy_static::lazy_static;
use tokio::io::AsyncWriteExt;
use tokio::sync::{Mutex as AsyncMutex, Mutex};
use crate::domain_initializer::initializer::{AcceptorTransmitterChannel, ReceiverTransmitterChannel};
use crate::response_generator::response_type::ResponseType;

use crate::transmitter::entity::transmit_data::TransmitData;
use crate::transmitter::repository::transmitter_repository::TransmitterRepository;

pub struct TransmitterRepositoryImpl {
    transmit_data: TransmitData,
    acceptor_transmitter_channel_arc: Option<Arc<AcceptorTransmitterChannel>>,
    receiver_transmitter_channel_arc: Option<Arc<ReceiverTransmitterChannel>>
}

impl TransmitterRepositoryImpl {
    pub fn new() -> Self {
        TransmitterRepositoryImpl {
            transmit_data: TransmitData::new(),
            acceptor_transmitter_channel_arc: None,
            receiver_transmitter_channel_arc: None
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
        let receiver_channel = self.receiver_transmitter_channel_arc.clone();

        let join_handle = tokio::task::spawn(async move {
            while let Some(stream_arc) = acceptor_channel.clone().expect("Need to inject channel").receive().await {
                println!("Transmitter transmit() loop");

                if let Some(receiver_transmitter_channel) = receiver_channel.clone() {
                    let response = receiver_transmitter_channel.receive().await;

                    if let Some(response) = response {
                        let mut client_socket_stream = stream_arc.lock().await;

                        // Acquire the lock on the Mutex to access the underlying data
                        let response_data = response.lock().await;

                        // Convert the response to JSON
                        let json_data = serde_json::to_string(&*response_data).expect("Failed to serialize to JSON");

                        // Print the data being transmitted
                        println!("Transmitting data: {}", json_data);

                        // Send the JSON data through the client socket stream
                        client_socket_stream.write_all(json_data.as_bytes()).await.expect("Failed to write to client");
                    }
                }

                tokio::time::sleep(Duration::from_millis(500)).await;
            }

            println!("Transmitter transmit() end");
        });

        join_handle.await.expect("Failed to await spawned task");

        // loop {
        //     println!("Transmitter transmit() test");
        //     tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        // }

        println!("TransmitterRepositoryImpl: transmit() end");
    }

    async fn inject_accept_transmitter_channel(&mut self, acceptor_transmitter_channel_arc: Arc<AcceptorTransmitterChannel>) {
        println!("TransmitterRepository: inject_accept_transmitter_channel()");

        self.acceptor_transmitter_channel_arc = Option::from(acceptor_transmitter_channel_arc);
    }

    async fn inject_receiver_transmitter_channel(&mut self, receiver_transmitter_channel_arc: Arc<ReceiverTransmitterChannel>) {
        println!("TransmitterRepository: inject_receiver_transmitter_channel()");

        self.receiver_transmitter_channel_arc = Option::from(receiver_transmitter_channel_arc);
    }
}

