use std::sync::Arc;
use std::time::Duration;
use async_trait::async_trait;
use futures::SinkExt;
use ipc_channel::ipc::IpcSender;
use lazy_static::lazy_static;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::sync::{Mutex as AsyncMutex, Mutex};
use crate::receiver::entity::receive_data::ReceiveData;
use crate::receiver::repository::server_receiver_repository::ServerReceiverRepository;
use crate::domain_initializer::initializer::{AcceptorReceiverChannel, AcceptorTransmitterChannel, ReceiverTransmitterLegacyChannel};

use serde_json::Value as JsonValue;
use crate::client_socket_accept::repository::client_socket_accept_repository_impl::ReceiverTransmitterChannel;
use crate::request_generator::test_generator::create_request_and_call_service;
use crate::response_generator::response_type::ResponseType;

pub struct ServerReceiverRepositoryImpl {
    receive_data: ReceiveData,
    acceptor_receiver_channel_arc: Option<Arc<AcceptorReceiverChannel>>,
    receiver_transmitter_channel_arc: Option<Arc<ReceiverTransmitterLegacyChannel>>
}

impl ServerReceiverRepositoryImpl {
    pub fn new() -> Self {
        ServerReceiverRepositoryImpl {
            receive_data: ReceiveData::new(),
            acceptor_receiver_channel_arc: None,
            receiver_transmitter_channel_arc: None
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<ServerReceiverRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<ServerReceiverRepositoryImpl>> =
                Arc::new(AsyncMutex::new(ServerReceiverRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }

    pub fn get_receive_data(&self) -> &ReceiveData {
        &self.receive_data
    }
}

#[async_trait]
impl ServerReceiverRepository for ServerReceiverRepositoryImpl {
    async fn receive(&mut self) {
        println!("ServerReceiverRepositoryImpl: receive()");

        let acceptor_channel = self.acceptor_receiver_channel_arc.clone();
        let receiver_transmitter_tx_clone = self.receiver_transmitter_channel_arc.clone();

        // let join_handle = tokio::task::spawn(async move {
        //     // Arc<Mutex<TcpStream>>
        //     while let Some(stream_arc) = acceptor_channel.clone().expect("Need to inject channel").receive().await {
        //         // Result<SocketAddr>
        //         let stream_result = stream_arc.lock().await.peer_addr();
        //
        //         tokio::select! {
        //             _ = futures::future::ready(stream_result.as_ref().clone()) => {
        //                 // SocketAddr
        //                 if let Some(peer_addr) = stream_result.ok() {
        //                     println!("Connected client address: {}", peer_addr);
        //                     // Arc<Mutex<TcpStream>>
        //                     let stream = stream_arc.clone();
        //                     // Option<Arc<ReceiverTransmitterChannel>>
        //                     let receiver_transmitter_tx_inner = receiver_transmitter_tx_clone.clone();
        //
        //                     tokio::spawn(async move {
        //                         handle_client(stream, receiver_transmitter_tx_inner).await;
        //                     });
        //                 } else {
        //                     eprintln!("Failed to get peer address");
        //                 }
        //             }
        //             else => {
        //                 eprintln!("Failed to get peer address");
        //             }
        //         }
        //     }
        //
        //     println!("Server Receiver Repository: client close socket");
        // });

        let join_handle = tokio::task::spawn(async move {
            // ClientSocket {
            //     address: String,
            //     stream: Arc<Mutex<TcpStream>>,
            //     each_client_receiver_transmitter_channel: Arc<ReceiverTransmitterChannel>,
            // }
            while let Some(client_socket) = acceptor_channel.clone().expect("Need to inject channel").receive().await {
                // let stream_result = stream_arc.lock().await.peer_addr();
                let receiver_transmitter_channel_arc = client_socket.each_client_receiver_transmitter_channel();
                let stream_arc = client_socket.stream();
                let stream_result = stream_arc.lock().await.peer_addr();

                tokio::select! {
                    _ = futures::future::ready(stream_result.as_ref().clone()) => {
                        if let Some(peer_addr) = stream_result.ok() {
                            println!("Connected client address: {}", peer_addr);
                            let stream = stream_arc.clone();
                            // let receiver_transmitter_tx_inner = receiver_transmitter_tx_clone.clone();
                            let receiver_transmitter_tx_inner = receiver_transmitter_channel_arc.clone();

                            tokio::spawn(async move {
                                handle_client(stream, receiver_transmitter_tx_inner).await;
                            });
                        } else {
                            eprintln!("Failed to get peer address");
                        }
                    }
                    else => {
                        eprintln!("Failed to get peer address");
                    }
                }
            }

            println!("Server Receiver Repository: client close socket");
        });

        join_handle.await.expect("Failed to await spawned task");

        println!("Server Receiver Repository: receive() end");
    }

    async fn inject_acceptor_receiver_channel(&mut self, acceptor_receiver_channel_arc: Arc<AcceptorReceiverChannel>) {
        println!("ServerReceiverRepositoryImpl: inject_acceptor_receiver_channel()");
        self.acceptor_receiver_channel_arc = Option::from(acceptor_receiver_channel_arc);
    }

    async fn inject_receiver_transmitter_channel(&mut self, receiver_transmitter_channel_arc: Arc<ReceiverTransmitterLegacyChannel>) {
        println!("ServerReceiverRepositoryImpl: inject_receiver_transmitter_channel()");
        self.receiver_transmitter_channel_arc = Option::from(receiver_transmitter_channel_arc);
    }
}

async fn handle_client(stream: Arc<Mutex<TcpStream>>, receiver_transmitter_tx: Arc<ReceiverTransmitterChannel>) {
    let mut buffer = vec![0; 1024]; // Adjust the buffer size as needed

    loop {
        match stream.lock().await.read(&mut buffer).await {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    break;
                }

                let stored_data = &buffer[..bytes_read];

                match serde_json::from_slice::<serde_json::Value>(stored_data) {
                    Ok(decoded_object) => {
                        println!("Received content: {:?}", decoded_object);

                        // TODO: This part could be cleaner; the loop logic should ideally go to the controller
                        // Option<ResponseType>
                        let response_option = create_request_and_call_service(&decoded_object).await;

                        // &ResponseType
                        if let Some(response) = &response_option {
                            println!("Generated Response Type: {:?}", response);
                        } else {
                            println!("Failed to generate Response Type");
                        }

                        // &Arc<ReceiverTransmitterChannel>
                        if let tx = &receiver_transmitter_tx {
                            tx.send(Arc::new(AsyncMutex::new(response_option.unwrap()))).await;
                            println!("handle_client: Sent response to Transmitter through channel");
                        }
                    }
                    Err(err) => {
                        println!("Error decoding JSON: {:?}", err);
                    }
                }
            }
            Err(err) => {
                // Handle read error
                println!("Error reading from stream: {:?}", err);
                break;
            }
        }

        tokio::time::sleep(Duration::from_millis(500)).await;
    }
}

// async fn handle_client(stream: Arc<Mutex<TcpStream>>, receiver_transmitter_tx: Option<Arc<ReceiverTransmitterChannel>>) {
//     let mut buffer = vec![0; 1024]; // Adjust the buffer size as needed
//
//     loop {
//         match stream.lock().await.read(&mut buffer).await {
//             Ok(bytes_read) => {
//                 if bytes_read == 0 {
//                     break;
//                 }
//
//                 let stored_data = &buffer[..bytes_read];
//
//                 match serde_json::from_slice::<serde_json::Value>(stored_data) {
//                     Ok(decoded_object) => {
//                         println!("Received content: {:?}", decoded_object);
//
//                         // TODO: This part could be cleaner; the loop logic should ideally go to the controller
//                         // Option<ResponseType>
//                         let response_option = create_request_and_call_service(&decoded_object).await;
//
//                         // &ResponseType
//                         if let Some(response) = &response_option {
//                             println!("Generated Response Type: {:?}", response);
//                         } else {
//                             println!("Failed to generate Response Type");
//                         }
//
//                         // &Arc<ReceiverTransmitterChannel>
//                         if let Some(tx) = &receiver_transmitter_tx {
//                             tx.send(Arc::new(AsyncMutex::new(response_option.unwrap()))).await;
//                             println!("handle_client: Sent response to Transmitter through channel");
//                         } else {
//                             println!("Error sending response through channel");
//                         }
//                     }
//                     Err(err) => {
//                         println!("Error decoding JSON: {:?}", err);
//                     }
//                 }
//             }
//             Err(err) => {
//                 // Handle read error
//                 println!("Error reading from stream: {:?}", err);
//                 break;
//             }
//         }
//
//         tokio::time::sleep(Duration::from_millis(500)).await;
//     }
// }

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use tokio::io::AsyncWriteExt;
    use super::*;
    use tokio::net::{TcpListener, TcpStream};
    use tokio::sync::Mutex;
    use crate::common::ip_address::local_ip_finder::IPAddress;
    use crate::domain_initializer::initializer::AcceptorReceiverChannel;

    // #[tokio::test]
    // async fn test_server_receiver_repository_receive() {
    //     let acceptor_receiver_channel = AcceptorReceiverChannel::new(1);
    //     let acceptor_receiver_channel_arc = Arc::new(acceptor_receiver_channel.clone());
    //
    //     let repository = ServerReceiverRepositoryImpl::new();
    //     let repository_mutex = Arc::new(AsyncMutex::new(repository));
    //
    //     let receiver_thread = tokio::spawn(async move {
    //         let ip = IPAddress::get_local_ip_from_google().unwrap();
    //         let binding = format!("{}:12345", ip).to_string();
    //         let address = binding.as_str();;
    //
    //         let listener = TcpListener::bind(address).await.expect("Failed to bind address");
    //         println!("server: bind Success");
    //
    //         let (mut stream, _) = listener.accept().await.expect("Failed to accept connection");
    //         println!("server: accept Success");
    //
    //         let acceptor_channel_arc_clone = acceptor_receiver_channel_arc.clone();
    //
    //         let mut repository_guard = repository_mutex.lock().await;
    //         repository_guard.inject_acceptor_receiver_channel(acceptor_channel_arc_clone).await;
    //
    //         acceptor_receiver_channel_arc.send(Arc::new(Mutex::new(stream))).await;
    //         repository_guard.receive().await;
    //     });
    //
    //     tokio::time::sleep(Duration::from_secs(2)).await;
    //
    //     let transmitter_thread = tokio::spawn(async move {
    //         let ip = IPAddress::get_local_ip_from_google().unwrap();
    //         let binding = format!("{}:12345", ip).to_string();
    //         let address = binding.as_str();;
    //
    //         match TcpStream::connect(address).await {
    //             Ok(mut stream) => {
    //                 println!("Success to connect!");
    //
    //                 let data_to_send = b"Hello, Rust Network Library: Tokio!";
    //                 if let Err(e) = stream.write_all(&data_to_send[..]).await {
    //                     eprintln!("Failed to write to stream: {:?}", e);
    //                 }
    //             }
    //             _ => {}
    //         }
    //     });
    //
    //     tokio::try_join!(receiver_thread, transmitter_thread).expect("Test failed");
    // }
}
