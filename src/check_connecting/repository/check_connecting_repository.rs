use std::sync::Arc;
use async_trait::async_trait;
use tokio::sync::Mutex;
use crate::client_socket_accept::entity::client_socket::ClientSocket;

#[async_trait]
pub trait CheckConnectingRepository {
    async fn send_message_for_checking_connect (&self, client_socket:ClientSocket);
    async fn find_account_unique_id_by_address (&self, address: &str) -> Option<i32>;
    async fn find_client_socket_by_account_unique_id (&self, account_id: i32) -> Option<ClientSocket>;
}