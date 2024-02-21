use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddCardListToHandResponse {
    is_success: bool,
}

impl AddCardListToHandResponse {
    pub fn new(is_success: bool) -> Self { AddCardListToHandResponse { is_success } }
    pub fn is_success(&self) -> bool { self.is_success }
}