use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SummarizeSupportCardEffectResponse {
    is_success: bool,
}

impl SummarizeSupportCardEffectResponse {
    pub fn new(is_success: bool) -> Self {
        SummarizeSupportCardEffectResponse { is_success }
    }
}
