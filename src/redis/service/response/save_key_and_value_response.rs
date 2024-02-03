use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveKeyAndValueResponse {
    is_success: bool,
}

impl SaveKeyAndValueResponse {
    pub fn new(is_success: bool) -> Self { SaveKeyAndValueResponse { is_success } }
}