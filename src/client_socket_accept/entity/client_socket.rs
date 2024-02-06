use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use crate::client_socket_accept::repository::client_socket_accept_repository_impl::ReceiverTransmitterChannel;

#[derive(Clone, Debug)]
pub struct ClientSocket {
    address: String,
    stream: Arc<Mutex<TcpStream>>,
    each_client_receiver_transmitter_channel: Arc<ReceiverTransmitterChannel>,
}

impl ClientSocket {
    pub fn new(address: String,
               stream: TcpStream,
               each_client_receiver_transmitter_channel: Arc<ReceiverTransmitterChannel>) -> Self {

        ClientSocket {
            address,
            stream: Arc::new(Mutex::new(stream)),
            each_client_receiver_transmitter_channel
        }
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn stream(&self) -> Arc<Mutex<TcpStream>> {
        self.stream.clone()
    }

    pub fn each_client_receiver_transmitter_channel(&self) -> &Arc<ReceiverTransmitterChannel> {
        &self.each_client_receiver_transmitter_channel
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::net::{TcpListener, TcpStream};

    // #[tokio::test]
    // async fn test_socket_client() {
    //     let listener = TcpListener::bind("127.0.0.1:7890").await.unwrap();
    //     let server_addr = listener.local_addr().unwrap();
    //     let client_stream = TcpStream::connect(&server_addr).await.unwrap();
    //
    //     let client = ClientSocket::new("test_client".to_string(), client_stream, );
    //
    //     assert_eq!(client.address(), "test_client");
    //
    //     drop(listener);
    // }
}
