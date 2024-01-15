use std::sync::Arc;
use async_trait::async_trait;
use crate::domain_initializer::initializer::{AcceptorReceiverChannel, AcceptorTransmitterChannel};

#[async_trait]
pub trait ClientSocketAcceptService: Send + Sync {
    async fn accept_client(&self);
    async fn inject_acceptor_receiver_channel(&self, acceptor_receiver_channel_arc: Arc<AcceptorReceiverChannel>);
    async fn inject_acceptor_transmitter_channel(&mut self, acceptor_transmitter_channel_arc: Arc<AcceptorTransmitterChannel>);
}