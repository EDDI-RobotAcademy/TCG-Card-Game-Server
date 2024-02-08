use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CalculateEffectResponse {
    is_success: bool,
}

impl CalculateEffectResponse {
    pub fn new(is_success: bool) -> Self {
        CalculateEffectResponse { is_success }
    }
}
