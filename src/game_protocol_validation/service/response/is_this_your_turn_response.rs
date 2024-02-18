use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IsThisYourTurnResponse {
    is_success: bool,
}

impl IsThisYourTurnResponse {
    pub fn new(is_success: bool) -> Self {
        IsThisYourTurnResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }

    pub fn get_is_success(&self) -> bool { self.is_success }
}