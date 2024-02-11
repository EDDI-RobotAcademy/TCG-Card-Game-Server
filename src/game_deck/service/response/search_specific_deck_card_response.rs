use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSpecificDeckCardResponse {
    is_success: bool,
}

impl SearchSpecificDeckCardResponse {
    pub fn new(is_success: bool) -> Self { SearchSpecificDeckCardResponse { is_success } }
    pub fn is_success(&self) -> bool { self.is_success }
}