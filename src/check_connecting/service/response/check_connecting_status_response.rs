use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckConnectingStatusResponse {
    is_success: bool,
}

impl CheckConnectingStatusResponse {
    pub fn new(is_success: bool) -> Self {
        CheckConnectingStatusResponse { is_success }
    }
}