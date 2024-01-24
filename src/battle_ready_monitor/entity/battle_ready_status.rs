use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum BattleReadyStatus {
    WAIT,
    SUCCESS,
    FAIL,
}