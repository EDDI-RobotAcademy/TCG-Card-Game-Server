use serde::{Deserialize, Serialize};
use crate::rock_paper_scissors::entity::rock_paper_scissors_result::RockPaperScissorsResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckRockPaperScissorsWinnerResponseForm {
    am_i_first_turn: RockPaperScissorsResult,
}

impl CheckRockPaperScissorsWinnerResponseForm {
    pub fn new(
        am_i_first_turn: RockPaperScissorsResult,

    ) -> Self {

        CheckRockPaperScissorsWinnerResponseForm {
            am_i_first_turn,
        }
    }
}