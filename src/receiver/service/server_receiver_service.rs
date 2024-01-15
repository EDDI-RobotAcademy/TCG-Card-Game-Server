use std::sync::Arc;
use async_trait::async_trait;
use crate::domain_initializer::initializer::{AcceptorReceiverChannel, AcceptorTransmitterChannel};

#[async_trait]
pub trait ServerReceiverService {
    async fn client_receive(&mut self);
    async fn inject_acceptor_receiver_channel(&mut self, acceptor_receiver_channel_arc: Arc<AcceptorReceiverChannel>);
}