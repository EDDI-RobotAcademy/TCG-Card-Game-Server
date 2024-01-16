use std::sync::Arc;
use async_trait::async_trait;
use ipc_channel::ipc::{IpcReceiver, IpcSender};
use crate::domain_initializer::initializer::AcceptorTransmitterChannel;
use crate::response_generator::response_type::ResponseType;

#[async_trait]
pub trait TransmitterController {
    async fn transmit_to_client(&mut self);
    async fn inject_acceptor_transmitter_channel(&mut self, acceptor_transmitter_channel_arc: Arc<AcceptorTransmitterChannel>);
    async fn inject_receiver_transmitter_channel(&mut self, receiver_transmitter_tx: IpcReceiver<ResponseType>);
}