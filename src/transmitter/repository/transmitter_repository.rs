use std::sync::Arc;
use async_trait::async_trait;
use ipc_channel::ipc::IpcReceiver;
use crate::domain_initializer::initializer::AcceptorTransmitterChannel;
use crate::response_generator::response_type::ResponseType;

#[async_trait]
pub trait TransmitterRepository {
    async fn transmit(&mut self);
    async fn inject_accept_transmitter_channel(&mut self, acceptor_transmitter_channel_arc: Arc<AcceptorTransmitterChannel>);
    async fn inject_receiver_transmitter_channel(&mut self, receiver_transmitter_tx: IpcReceiver<ResponseType>);
}