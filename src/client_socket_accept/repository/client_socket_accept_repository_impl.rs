use std::collections::HashMap;
use tokio::net::TcpListener;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Mutex as AsyncMutex, Mutex};
use async_trait::async_trait;
use lazy_static::lazy_static;
use crate::client_socket_accept::entity::client_socket::ClientSocket;
use crate::client_socket_accept::repository::client_socket_accept_repository::ClientSocketAcceptRepository;
use crate::define_channel;
use crate::domain_initializer::initializer::{AcceptorReceiverChannel, AcceptorTransmitterChannel};
use crate::response_generator::response_type::ResponseType;

use tokio::sync::mpsc;
define_channel!(ReceiverTransmitterChannel, Arc<Mutex<ResponseType>>);

#[derive(Clone)]
pub struct ClientSocketAcceptRepositoryImpl {
    client_list: Arc<AsyncMutex<HashMap<String, ClientSocket>>>,
    acceptor_receiver_channel_arc: Option<Arc<AcceptorReceiverChannel>>,
    acceptor_transmitter_channel_arc: Option<Arc<AcceptorTransmitterChannel>>
}

impl ClientSocketAcceptRepositoryImpl {
    pub fn new() -> Self {
        ClientSocketAcceptRepositoryImpl {
            client_list: Arc::new(AsyncMutex::new(HashMap::new())),
            acceptor_receiver_channel_arc: None,
            acceptor_transmitter_channel_arc: None
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<ClientSocketAcceptRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<ClientSocketAcceptRepositoryImpl>> =
                Arc::new(AsyncMutex::new(ClientSocketAcceptRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }

    pub fn get_client_list(&self) -> &Arc<AsyncMutex<HashMap<String, ClientSocket>>> {
        &self.client_list
    }
}

#[async_trait]
impl ClientSocketAcceptRepository for ClientSocketAcceptRepositoryImpl {

    async fn accept_client(&mut self, listener: &TcpListener) {
        println!("Client Socket Accept Repository: accept()");
        loop {
            match listener.accept().await {
                Ok((stream, peer_addr)) => {
                    println!("Accepted client from: {}", peer_addr);

                    let each_client_receiver_transmitter_channel = ReceiverTransmitterChannel::new(1);
                    let each_client_receiver_transmitter_channel_arc = Arc::new(each_client_receiver_transmitter_channel.clone());

                    let client = ClientSocket::new(peer_addr.to_string(), stream, each_client_receiver_transmitter_channel_arc);
                    let mut client_list_gaurd = self.client_list.lock().await;

                    client_list_gaurd.insert(client.address().to_string(), client.clone());

                    if let Some(acceptor_receiver_channel) = &self.acceptor_receiver_channel_arc {
                        tokio::time::sleep(Duration::from_millis(100)).await;

                        // let stream = client.stream();
                        // let _ = acceptor_receiver_channel.send(stream).await;
                        let _ = acceptor_receiver_channel.send(client).await;
                        println!("send socket info to receiver with ipc channel");
                    } else {
                        eprintln!("Acceptor Receiver channel is not initialized");
                    }

                    if let Some(acceptor_transmitter_channel) = &self.acceptor_transmitter_channel_arc {
                        tokio::time::sleep(Duration::from_millis(100)).await;

                        // let stream = client.stream();
                        // let _ = acceptor_transmitter_channel.send(stream).await;
                        println!("send socket info to transmitter with ipc channel");
                    } else {
                        eprintln!("Acceptor Transmitter channel is not initialized");
                    }
                }
                Err(err) => {
                    eprintln!("Error accepting client: {:?}", err);
                }
            }

            tokio::time::sleep(Duration::from_millis(300)).await;
        }
    }

    async fn inject_acceptor_receiver_channel(&mut self, acceptor_receiver_channel_arc: Arc<AcceptorReceiverChannel>) {
        self.acceptor_receiver_channel_arc = Option::from(acceptor_receiver_channel_arc);
    }

    async fn inject_acceptor_transmitter_channel(&mut self, acceptor_transmitter_channel_arc: Arc<AcceptorTransmitterChannel>) {
        self.acceptor_transmitter_channel_arc = Option::from(acceptor_transmitter_channel_arc);
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use super::*;

    #[tokio::test]
    async fn test_singleton_behavior() {
        let instance1 = ClientSocketAcceptRepositoryImpl::get_instance();
        let instance2 = ClientSocketAcceptRepositoryImpl::get_instance();

        assert!(Arc::ptr_eq(&instance1, &instance2));
    }

    #[tokio::test]
    async fn test_accept_client() {
        let listener = Arc::new(TcpListener::bind("127.0.0.1:7128").await.expect("Failed to bind to address"));
        let repository = ClientSocketAcceptRepositoryImpl::get_instance();
        let listener_clone = listener.clone();

        let acceptor_receiver_channel = AcceptorReceiverChannel::new(1);
        let acceptor_receiver_channel_arc = Arc::new(acceptor_receiver_channel.clone());

        tokio::spawn(async move {
            let acceptor_receiver_channel_arc_clone = acceptor_receiver_channel_arc.clone();

            let mut repository_gaurd = repository.lock().await;
            repository_gaurd.inject_acceptor_receiver_channel(acceptor_receiver_channel_arc_clone).await;
            repository_gaurd.accept_client(&listener_clone).await;
        });

        tokio::time::sleep(Duration::from_secs(5)).await;

        assert!(listener.local_addr().is_ok());
    }
}
