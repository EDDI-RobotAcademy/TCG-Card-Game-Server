use serde::{Deserialize, Serialize};
use crate::first_turn_decision_ready_account_hash::entity::first_turn_decision_ready_account_hash_status::FirstTurnDecisionReadyAccountHashStatus;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirstTurnDecisionReadyAccountHashResponse {
    current_status: FirstTurnDecisionReadyAccountHashStatus,
}

impl FirstTurnDecisionReadyAccountHashResponse {
    pub fn new(current_status: FirstTurnDecisionReadyAccountHashStatus) -> Self {
        FirstTurnDecisionReadyAccountHashResponse { current_status }
    }
}
