use serde::{Deserialize, Serialize};
use crate::check_connecting::entity::check_connecting_status::ConnectStatus;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckConnectingMessageForm {
    connect_status: ConnectStatus
}

impl CheckConnectingMessageForm {
    pub fn new(connect_status: ConnectStatus) -> Self {
        CheckConnectingMessageForm {
            connect_status,
        }
    }
}