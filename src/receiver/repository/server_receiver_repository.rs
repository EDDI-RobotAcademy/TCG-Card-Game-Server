use std::sync::Arc;
use async_trait::async_trait;
use crate::domain_initializer::initializer::AcceptorReceiverChannel;

#[async_trait]
pub trait ServerReceiverRepository {
    async fn receive(&mut self);
    async fn inject_accept_channel(&mut self, acceptor_receiver_channel_arc: Arc<AcceptorReceiverChannel>);
}