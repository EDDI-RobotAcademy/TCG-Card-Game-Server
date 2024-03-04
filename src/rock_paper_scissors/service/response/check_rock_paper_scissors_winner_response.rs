use serde::{Deserialize, Serialize};
use crate::rock_paper_scissors::entity::rock_paper_scissors_result::RockPaperScissorsResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckRockPaperScissorsWinnerResponse{
    am_i_winner: RockPaperScissorsResult,
}

impl CheckRockPaperScissorsWinnerResponse {
    pub fn new(am_i_winner: RockPaperScissorsResult) -> Self {
        CheckRockPaperScissorsWinnerResponse {
            am_i_winner
        }
    }

    pub fn get_am_i_winner(&self) -> RockPaperScissorsResult {
        self.am_i_winner.clone()
    }
}