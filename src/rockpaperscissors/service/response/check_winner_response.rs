use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckWinnerResponse{
    am_i_winner: bool,
    check_draw_result:bool
}

impl CheckWinnerResponse {
    pub fn new(am_i_winner: bool, check_draw_result:bool) -> Self {
        CheckWinnerResponse
        { am_i_winner,
            check_draw_result }
    }
    pub fn get_am_i_winner(&self) -> bool { self.am_i_winner }
    pub fn get_check_draw_result(&self) -> bool { self.check_draw_result }
}