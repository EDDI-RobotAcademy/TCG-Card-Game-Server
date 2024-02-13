use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirstTurnDecisionWaitQueueResponseForm {
    is_success: bool,
}

impl FirstTurnDecisionWaitQueueResponseForm {
    pub fn new(is_success: bool) -> Self {
        FirstTurnDecisionWaitQueueResponseForm { is_success }
    }
}