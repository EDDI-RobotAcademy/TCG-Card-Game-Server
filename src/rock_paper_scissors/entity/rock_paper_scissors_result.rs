use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RockPaperScissorsResult {
    WAIT,
    WIN,
    LOSE,
}