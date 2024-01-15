use std::sync::Arc;
use async_trait::async_trait;
use crate::domain_initializer::initializer::AcceptorTransmitterChannel;

#[async_trait]
pub trait TransmitterRepository {
    async fn transmit(&mut self);
    async fn inject_accept_transmitter_channel(&mut self, acceptor_transmitter_channel_arc: Arc<AcceptorTransmitterChannel>);
}