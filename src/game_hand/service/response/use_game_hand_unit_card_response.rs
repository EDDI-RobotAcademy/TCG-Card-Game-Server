use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UseGameHandUnitCardResponse {
    is_success: bool,
}

impl UseGameHandUnitCardResponse {
    pub fn new(is_success: bool) -> Self {
        UseGameHandUnitCardResponse { is_success }
    }
}
