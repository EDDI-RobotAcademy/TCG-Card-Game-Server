use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleWaitQueueResponse {
    is_success: bool,
}

impl BattleWaitQueueResponse {
    pub fn new(is_success: bool) -> Self {
        BattleWaitQueueResponse { is_success }
    }
}
