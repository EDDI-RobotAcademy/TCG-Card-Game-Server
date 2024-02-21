use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurrenderResponse {
    is_success: bool,
}

impl SurrenderResponse {
    pub fn new(is_success: bool) -> Self {
        SurrenderResponse { is_success }
    }
}