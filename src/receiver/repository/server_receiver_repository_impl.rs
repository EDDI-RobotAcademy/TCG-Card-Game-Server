use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::sync::{Mutex as AsyncMutex, Mutex};
use crate::receiver::entity::receive_data::ReceiveData;
use crate::receiver::repository::server_receiver_repository::ServerReceiverRepository;
use crate::domain_initializer::initializer::AcceptorReceiverChannel;

use serde_json::Value as JsonValue;
use crate::request_generator::test_generator::create_account_request_and_call_service;

pub struct ServerReceiverRepositoryImpl {
    receive_data: ReceiveData,
    acceptor_receiver_channel_arc: Option<Arc<AcceptorReceiverChannel>>,
}

impl ServerReceiverRepositoryImpl {
    pub fn new() -> Self {
        ServerReceiverRepositoryImpl {
            receive_data: ReceiveData::new(),
            acceptor_receiver_channel_arc: None,
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
        println!("Server Receiver Repository: receive()");

        let acceptor_channel = self.acceptor_receiver_channel_arc.clone();

        let join_handle = tokio::task::spawn(async move {
            while let Some(stream_arc) = acceptor_channel.clone().expect("Need to inject channel").receive().await {
                let stream_result = stream_arc.lock().await.peer_addr();

                tokio::select! {
                _ = futures::future::ready(stream_result.as_ref().clone()) => {
                    if let Some(peer_addr) = stream_result.ok() {
                        println!("Connected client address: {}", peer_addr);
                        let stream = stream_arc.clone();

                        tokio::spawn(async move {
                            handle_client(stream).await;
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


    async fn inject_accept_channel(&mut self, acceptor_receiver_channel_arc: Arc<AcceptorReceiverChannel>) {
        self.acceptor_receiver_channel_arc = Option::from(acceptor_receiver_channel_arc);
    }
}

async fn handle_client(stream: Arc<Mutex<TcpStream>>) {
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
                        create_account_request_and_call_service(&decoded_object).await;
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
    }
}

// fn explore_json_value(value: JsonValue) {
//     match value {
//         JsonValue::Bool(v) => println!("Boolean: {:?}", v),
//         JsonValue::Number(v) => println!("Number: {:?}", v),
//         JsonValue::String(v) => println!("String: {:?}", v),
//         JsonValue::Array(array) => {
//             println!("Array: {:?}", array);
//             for item in array {
//                 explore_json_value(item);
//             }
//         }
//         JsonValue::Object(map) => {
//             println!("Object: {:?}", map);
//             for (key, value) in map {
//                 println!("Key: {:?}", key);
//                 explore_json_value(value);
//             }
//         }
//         JsonValue::Null => println!("Null"),
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

    #[tokio::test]
    async fn test_server_receiver_repository_receive() {
        let acceptor_receiver_channel = AcceptorReceiverChannel::new(1);
        let acceptor_receiver_channel_arc = Arc::new(acceptor_receiver_channel.clone());

        let repository = ServerReceiverRepositoryImpl::new();
        let repository_mutex = Arc::new(AsyncMutex::new(repository));

        let receiver_thread = tokio::spawn(async move {
            let ip = IPAddress::get_local_ip_from_google().unwrap();
            let binding = format!("{}:12345", ip).to_string();
            let address = binding.as_str();;

            let listener = TcpListener::bind(address).await.expect("Failed to bind address");
            println!("server: bind Success");

            let (mut stream, _) = listener.accept().await.expect("Failed to accept connection");
            println!("server: accept Success");

            let acceptor_channel_arc_clone = acceptor_receiver_channel_arc.clone();

            let mut repository_guard = repository_mutex.lock().await;
            repository_guard.inject_accept_channel(acceptor_channel_arc_clone).await;

            acceptor_receiver_channel_arc.send(Arc::new(Mutex::new(stream))).await;
            repository_guard.receive().await;
        });

        tokio::time::sleep(Duration::from_secs(2)).await;

        let transmitter_thread = tokio::spawn(async move {
            let ip = IPAddress::get_local_ip_from_google().unwrap();
            let binding = format!("{}:12345", ip).to_string();
            let address = binding.as_str();;

            match TcpStream::connect(address).await {
                Ok(mut stream) => {
                    println!("Success to connect!");

                    let data_to_send = b"Hello, Rust Network Library: Tokio!";
                    if let Err(e) = stream.write_all(&data_to_send[..]).await {
                        eprintln!("Failed to write to stream: {:?}", e);
                    }
                }
                _ => {}
            }
        });

        tokio::try_join!(receiver_thread, transmitter_thread).expect("Test failed");
    }
}
