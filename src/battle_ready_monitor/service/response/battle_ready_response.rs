use serde::{Deserialize, Serialize};
use crate::battle_ready_monitor::entity::battle_ready_status::BattleReadyStatus;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleReadyResponse {
    current_status: BattleReadyStatus,
}

impl BattleReadyResponse {
    pub fn new(current_status: BattleReadyStatus) -> Self {
        BattleReadyResponse { current_status }
    }
}
