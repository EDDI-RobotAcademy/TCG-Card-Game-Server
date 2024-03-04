use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckRockPaperScissorsWinnerResponseForm {
    am_i_first_turn: String,
}

impl CheckRockPaperScissorsWinnerResponseForm {
    pub fn new(
        am_i_first_turn: String,
    ) -> Self {

        CheckRockPaperScissorsWinnerResponseForm {
            am_i_first_turn,
        }
    }
}