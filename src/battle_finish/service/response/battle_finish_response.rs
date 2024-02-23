use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleFinishResponse {
    is_success: bool,
}

impl BattleFinishResponse {
    pub fn new(is_success: bool) -> Self {
        BattleFinishResponse { is_success }
    }
}