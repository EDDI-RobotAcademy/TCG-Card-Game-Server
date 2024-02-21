use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckRockpaperscissorsWinnerResponse{
    am_i_winner: String,

}

impl CheckRockpaperscissorsWinnerResponse {
    pub fn new(am_i_winner: String) -> Self {
        CheckRockpaperscissorsWinnerResponse
        { am_i_winner }
    }
    pub fn get_am_i_winner(&self) -> &str { &self.am_i_winner }

}