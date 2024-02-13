use serde::{Deserialize, Serialize};
use crate::game_turn::controller::request_form::first_turn_decision_request_form::FirstTurnDecisionRequestForm;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirstTurnDecisionResponseForm {
    first_player:i32,
    am_i_first_turn:bool,
    check_draw_result:bool
}

impl FirstTurnDecisionResponseForm {
    pub fn new(first_player: i32,am_i_first_turn:bool,check_draw_result:bool) -> Self {
        FirstTurnDecisionResponseForm {
            first_player,
            am_i_first_turn,
            check_draw_result
        }
    }

}