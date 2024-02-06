use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecideFirstTurnResponse {
    pub(crate) first_player:i32,
    pub(crate) result_is_draw:bool,

}

impl DecideFirstTurnResponse {
    pub fn new(first_player: i32, result_is_draw: bool) -> Self {
        DecideFirstTurnResponse {
            first_player,
            result_is_draw,
        }
    }
    pub fn get_first_player(self) -> i32 { self.first_player }

    pub fn get_result_is_draw(&self) -> bool { self.result_is_draw }
}