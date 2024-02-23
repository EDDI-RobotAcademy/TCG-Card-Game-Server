use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleMatchCancelResponse {
    is_success: bool,
}

impl BattleMatchCancelResponse {
    pub fn new(is_success: bool) -> Self {
        BattleMatchCancelResponse { is_success }
    }
}
