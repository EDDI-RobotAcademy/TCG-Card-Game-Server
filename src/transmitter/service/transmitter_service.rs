use std::sync::Arc;
use async_trait::async_trait;
use crate::domain_initializer::initializer::AcceptorTransmitterChannel;

#[async_trait]
pub trait TransmitterService {
    async fn transmit_to_client(&mut self);
    async fn inject_acceptor_transmitter_channel(&mut self, acceptor_transmitter_channel_arc: Arc<AcceptorTransmitterChannel>);
}