use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveDailyKeyAndValueResponse {
    is_success: bool,
}

impl SaveDailyKeyAndValueResponse {
    pub fn new(is_success: bool) -> Self { SaveDailyKeyAndValueResponse { is_success } }
}