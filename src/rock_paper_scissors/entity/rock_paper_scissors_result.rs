use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RockPaperScissorsResult {
    Wait = 0,
    Win = 1,
    Lose = 2,
}

impl From<i32> for RockPaperScissorsResult {
    fn from(value: i32) -> Self {
        match value {
            1 => RockPaperScissorsResult::Win,
            2 => RockPaperScissorsResult::Lose,
            _ => RockPaperScissorsResult::Wait,
        }
    }
}
