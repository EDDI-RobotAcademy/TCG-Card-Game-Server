use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConnectStatus {
    CONNECTED,
    DISCONNECTED,
    DUMMY,
}