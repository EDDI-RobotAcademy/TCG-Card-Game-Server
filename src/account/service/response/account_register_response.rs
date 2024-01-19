use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountRegisterResponse {
    is_success: bool,
}

impl AccountRegisterResponse {
    pub fn new(is_success: bool) -> Self {
        AccountRegisterResponse { is_success }
    }
}
