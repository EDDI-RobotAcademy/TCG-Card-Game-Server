use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NextTurnResponse {
    is_success: bool
}

impl NextTurnResponse {
    pub fn new(is_success: bool) -> Self {
        NextTurnResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}
