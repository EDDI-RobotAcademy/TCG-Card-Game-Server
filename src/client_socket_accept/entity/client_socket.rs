use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct ClientSocket {
    address: String,
    stream: Arc<Mutex<TcpStream>>,
}

impl ClientSocket {
    pub fn new(address: String, stream: TcpStream) -> Self {
        ClientSocket {
            address,
            stream: Arc::new(Mutex::new(stream)),
        }
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn stream(&self) -> Arc<Mutex<TcpStream>> {
        self.stream.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::net::{TcpListener, TcpStream};

    #[tokio::test]
    async fn test_socket_client() {
        let listener = TcpListener::bind("127.0.0.1:7890").await.unwrap();
        let server_addr = listener.local_addr().unwrap();
        let client_stream = TcpStream::connect(&server_addr).await.unwrap();

        let client = ClientSocket::new("test_client".to_string(), client_stream);

        assert_eq!(client.address(), "test_client");

        drop(listener);
    }
}
