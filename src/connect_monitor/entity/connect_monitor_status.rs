use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConnectMonitorStatus {
    CONNECTED,
    DISCONNECTED,
    DUMMY,
}