use std::sync::Arc;
use tokio::net::TcpListener;

use async_trait::async_trait;
use crate::domain_initializer::initializer::{AcceptorReceiverChannel, AcceptorTransmitterChannel};

#[async_trait]
pub trait ClientSocketAcceptRepository {
    async fn accept_client(&mut self, listener: &TcpListener);
    async fn inject_acceptor_receiver_channel(&mut self, acceptor_receiver_channel_arc: Arc<AcceptorReceiverChannel>);
    async fn inject_acceptor_transmitter_channel(&mut self, acceptor_transmitter_channel_arc: Arc<AcceptorTransmitterChannel>);
}