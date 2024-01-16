use std::sync::Arc;
use async_trait::async_trait;
use ipc_channel::ipc::IpcSender;
use crate::domain_initializer::initializer::{AcceptorReceiverChannel, AcceptorTransmitterChannel, ReceiverTransmitterChannel};
use crate::response_generator::response_type::ResponseType;

#[async_trait]
pub trait ServerReceiverService {
    async fn client_receive(&mut self);
    async fn inject_acceptor_receiver_channel(&mut self, acceptor_receiver_channel_arc: Arc<AcceptorReceiverChannel>);
    async fn inject_receiver_transmitter_channel(&mut self, receiver_transmitter_channel_arc: Arc<ReceiverTransmitterChannel>);
}