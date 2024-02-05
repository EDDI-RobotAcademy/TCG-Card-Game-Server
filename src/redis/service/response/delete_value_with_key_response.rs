use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteValueWithKeyResponse {
    is_success: bool,
}

impl DeleteValueWithKeyResponse {
    pub fn new(is_success: bool) -> Self { DeleteValueWithKeyResponse { is_success } }
}