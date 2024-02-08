use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CheckCardsFromHandResponse {
    is_success: bool,
}

impl CheckCardsFromHandResponse {
    pub fn new(is_success: bool) -> Self {
        CheckCardsFromHandResponse { is_success }
    }

    pub fn get_is_success(&self) -> bool {
        self.is_success
    }
}