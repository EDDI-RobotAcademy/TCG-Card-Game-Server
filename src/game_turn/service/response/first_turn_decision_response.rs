use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirstTurnDecisionResponse {
     first_player:i32,
     am_i_first_player:bool,
     check_draw_result:bool,

}

impl FirstTurnDecisionResponse {
    pub fn new(first_player: i32, check_draw_result: bool,am_i_first_player:bool,) -> Self {
        FirstTurnDecisionResponse {
            first_player,
            am_i_first_player,
            check_draw_result,
        }
    }
    pub fn get_first_player(self) -> i32 { self.first_player }
    pub fn get_am_i_first_player(&self) -> bool { self.am_i_first_player }
    pub fn get_result_is_draw(&self) -> bool { self.check_draw_result }
}