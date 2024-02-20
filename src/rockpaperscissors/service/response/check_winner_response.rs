use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckWinnerResponse{
    am_i_winner: bool,

}

impl CheckWinnerResponse {
    pub fn new(am_i_winner: bool) -> Self {
        CheckWinnerResponse
        { am_i_winner }
    }
    pub fn get_am_i_winner(&self) -> bool { self.am_i_winner }

}