use std::collections::HashMap;
use crate::client_socket_accept::entity::client_socket::ClientSocket;

#[derive(Debug)]
pub struct ClientSocketList {
    pub client_socket_list: HashMap<String, ClientSocket>,
}

impl ClientSocketList {
    pub fn new() -> ClientSocketList {
        ClientSocketList { client_socket_list: HashMap::new() }
    }

    pub fn get_client_socket_list(&self) -> &HashMap<String, ClientSocket> {
        &self.client_socket_list
    }

    pub fn add_client_socket(&mut self, address: String, client_socket: ClientSocket) {
        self.client_socket_list.insert(address, client_socket);
    }

    pub fn remove_client_socket(&mut self, address:&str) {
        self.client_socket_list.remove(address);
    }
}
