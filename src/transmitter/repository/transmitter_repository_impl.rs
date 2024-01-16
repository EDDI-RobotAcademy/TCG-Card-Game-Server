use std::sync::Arc;
use async_trait::async_trait;
use ipc_channel::ipc::IpcReceiver;
use lazy_static::lazy_static;
use tokio::sync::{Mutex as AsyncMutex, Mutex};
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

        // let join_handle = tokio::task::spawn(async move {
        //     while let Some(stream_arc) = acceptor_channel.clone().expect("Need to inject channel").receive().await {
        //         println!("Transmitter transmit() loop");
        //
        //         let acceptor_transmitter_channel_arc = self.acceptor_transmitter_channel_arc.clone();
        //         let receiver_transmitter_tx = self.receiver_transmitter_tx.as_ref();
        //
        //         tokio::spawn(async move {
        //             let acceptor_transmitter_channel = acceptor_transmitter_channel_arc;
        //
        //             // Wrap the receiver in a Mutex to make it thread-safe
        //             let receiver_transmitter_tx = receiver_transmitter_tx.map(|rx| Arc::new(Mutex::new(rx)));
        //
        //             let receiver_transmitter_tx = receiver_transmitter_tx.map(|rx| {
        //                 Arc::new(Mutex::new(rx.unwrap_or_else(Default::default)))
        //             });
        //
        //
        //
        //             // Lock the Mutex to get the inner receiver
        //             let receiver_transmitter_tx = receiver_transmitter_tx.lock().await;
        //
        //             // Use the receiver as needed
        //
        //             tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        //         });
        //     }
        //
        //     println!("Transmitter transmit() end");
        // });
        //
        // join_handle.await.expect("Failed to await spawned task");
        loop {
            println!("Transmitter transmit() test");
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }

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

