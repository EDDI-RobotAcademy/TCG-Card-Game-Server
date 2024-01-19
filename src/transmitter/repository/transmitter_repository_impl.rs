use std::sync::Arc;
use std::time::Duration;
use async_trait::async_trait;
use ipc_channel::ipc::IpcReceiver;
use lazy_static::lazy_static;
use tokio::io::AsyncWriteExt;
use tokio::sync::{Mutex as AsyncMutex, Mutex};
use crate::domain_initializer::initializer::{AcceptorTransmitterChannel, ReceiverTransmitterLegacyChannel};
use crate::response_generator::response_type::ResponseType;

use crate::transmitter::entity::transmit_data::TransmitData;
use crate::transmitter::repository::transmitter_repository::TransmitterRepository;

pub struct TransmitterRepositoryImpl {
    transmit_data: TransmitData,
    acceptor_transmitter_channel_arc: Option<Arc<AcceptorTransmitterChannel>>,
    receiver_transmitter_channel_arc: Option<Arc<ReceiverTransmitterLegacyChannel>>
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

        // Arc<Mutex<TcpStream>>
        // while let Some(stream_arc) = acceptor_channel.clone().expect("Need to inject channel").receive().await {
        //     // Option<Arc<ReceiverTransmitterLegacyChannel>>
        //     let receiver_channel_clone = receiver_channel.clone();
        //
        //     // 변경된 부분: tokio::task::spawn을 사용하여 각 클라이언트에 대한 통신을 별도의 태스크로 분리
        //     tokio::task::spawn(async move {
        //         println!("Transmitter transmit() loop");
        //
        //         // Arc<ReceiverTransmitterLegacyChannel>
        //         if let Some(receiver_transmitter_channel) = receiver_channel_clone {
        //             loop {
        //                 // 변경된 부분: 각 클라이언트에 대한 비동기 통신 작업을 별도의 태스크로 생성하여 동시에 수행
        //                 // Option<Arc<Mutex<ResponseType>>>
        //                 let response = receiver_transmitter_channel.receive().await;
        //                 println!("Transmitter receive channel data from Receiver");
        //
        //                 // Arc<Mutex<ResponseType>>
        //                 if let Some(response) = response {
        //                     // MutexGuard<TcpStream>
        //                     let mut client_socket_stream = stream_arc.lock().await;
        //                     println!("Transmitter lock socket");
        //
        //                     // MutexGaurd<ResponseType>
        //                     let response_data = response.lock().await;
        //                     let json_data = serde_json::to_string(&*response_data).expect("Failed to serialize to JSON");
        //
        //                     println!("Transmitting data: {}", json_data);
        //
        //                     client_socket_stream.write_all(json_data.as_bytes()).await.expect("Failed to write to client");
        //                 }
        //
        //                 tokio::time::sleep(Duration::from_millis(500)).await;
        //             }
        //         }
        //
        //         println!("Transmitter transmit() loop finished");
        //     });
        // }

        while let Some(client_socket) = acceptor_channel.clone().expect("Need to inject channel").receive().await {
            // ClientSocket {
            //     address: String,
            //     stream: Arc<Mutex<TcpStream>>,
            //     each_client_receiver_transmitter_channel: Arc<ReceiverTransmitterChannel>,
            // }
            let stream_arc = client_socket.stream();
            let receiver_transmitter_channel_arc = client_socket.each_client_receiver_transmitter_channel();
            let receiver_transmitter_channel_clone = receiver_transmitter_channel_arc.clone();

            // 변경된 부분: tokio::task::spawn을 사용하여 각 클라이언트에 대한 통신을 별도의 태스크로 분리
            tokio::task::spawn(async move {
                println!("Transmitter transmit() loop");

                // Arc<ReceiverTransmitterLegacyChannel>
                if let receiver_transmitter_channel = receiver_transmitter_channel_clone {
                    loop {
                        // 변경된 부분: 각 클라이언트에 대한 비동기 통신 작업을 별도의 태스크로 생성하여 동시에 수행
                        // Option<Arc<Mutex<ResponseType>>>
                        let response = receiver_transmitter_channel.receive().await;
                        println!("Transmitter receive channel data from Receiver");

                        // Arc<Mutex<ResponseType>>
                        if let Some(response) = response {
                            // MutexGuard<TcpStream>
                            let mut client_socket_stream = stream_arc.lock().await;
                            println!("Transmitter lock socket");

                            // MutexGaurd<ResponseType>
                            let response_data = response.lock().await;
                            let json_data = serde_json::to_string(&*response_data).expect("Failed to serialize to JSON");

                            println!("Transmitting data: {}", json_data);

                            client_socket_stream.write_all(json_data.as_bytes()).await.expect("Failed to write to client");
                        }

                        tokio::time::sleep(Duration::from_millis(500)).await;
                    }
                }

                println!("Transmitter transmit() loop finished");
            });
        }

        println!("TransmitterRepositoryImpl: transmit() end");
    }

    async fn inject_accept_transmitter_channel(&mut self, acceptor_transmitter_channel_arc: Arc<AcceptorTransmitterChannel>) {
        println!("TransmitterRepository: inject_accept_transmitter_channel()");

        self.acceptor_transmitter_channel_arc = Option::from(acceptor_transmitter_channel_arc);
    }

    async fn inject_receiver_transmitter_channel(&mut self, receiver_transmitter_channel_arc: Arc<ReceiverTransmitterLegacyChannel>) {
        println!("TransmitterRepository: inject_receiver_transmitter_channel()");

        self.receiver_transmitter_channel_arc = Option::from(receiver_transmitter_channel_arc);
    }
}

