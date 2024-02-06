use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutCardsOnDeckResponse {
    is_success: bool,
}

impl PutCardsOnDeckResponse {
    pub fn new(is_success: bool) -> Self { PutCardsOnDeckResponse { is_success } }
    pub fn get_is_success(&self) -> bool { self.is_success }
}