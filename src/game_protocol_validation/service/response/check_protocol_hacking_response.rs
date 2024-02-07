use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CheckProtocolHackingResponse {
    is_success: bool,
}

impl CheckProtocolHackingResponse {
    pub fn new(is_success: bool) -> Self {
        CheckProtocolHackingResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}
