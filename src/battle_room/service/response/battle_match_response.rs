use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleMatchResponse {
    is_success: bool,
}

impl BattleMatchResponse {
    pub fn new(is_success: bool) -> Self {
        BattleMatchResponse { is_success }
    }
}
