use std::sync::Arc;
use async_trait::async_trait;
use crate::domain_initializer::initializer::AcceptorReceiverChannel;

#[async_trait]
pub trait ClientSocketAcceptController: Send + Sync {
    async fn accept_client(&self);
    async fn inject_accept_channel(&self, acceptor_receiver_channel_arc: Arc<AcceptorReceiverChannel>);
}