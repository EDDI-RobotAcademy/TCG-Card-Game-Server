use serde::{Deserialize, Serialize};
use crate::battle_ready_account_hash::entity::battle_ready_account_hash_status::BattleReadyAccountHashStatus;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleReadyAccountHashResponse {
    current_status: BattleReadyAccountHashStatus,
}

impl BattleReadyAccountHashResponse {
    pub fn new(current_status: BattleReadyAccountHashStatus) -> Self {
        BattleReadyAccountHashResponse { current_status }
    }
}
