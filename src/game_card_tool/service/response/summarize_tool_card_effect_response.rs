use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SummarizeToolCardEffectResponse {
    is_success: bool,
}

impl SummarizeToolCardEffectResponse {
    pub fn new(is_success: bool) -> Self {
        SummarizeToolCardEffectResponse { is_success }
    }
}