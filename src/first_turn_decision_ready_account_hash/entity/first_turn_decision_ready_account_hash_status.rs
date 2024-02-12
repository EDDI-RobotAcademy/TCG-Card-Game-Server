use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum FirstTurnDecisionReadyAccountHashStatus {
    WAIT,
    PREPARE,
    PREPARE_PROCESS,
    SUCCESS,
    FAIL,
}