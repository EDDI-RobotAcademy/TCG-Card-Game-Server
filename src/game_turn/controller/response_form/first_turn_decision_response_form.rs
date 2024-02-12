use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirstTurnDecisionResponseForm {
    first_player_session_id: String,
    result_is_draw: bool,
}

impl FirstTurnDecisionResponseForm {
    pub fn new(first_player_session_id: String, result_is_draw: bool) -> Self {
        FirstTurnDecisionResponseForm {
            first_player_session_id,
            result_is_draw
        }
    }
    pub fn get_first_player(&self) -> &str { &self.first_player_session_id }
    pub fn get_result_is_draw(self) -> bool { self.result_is_draw }
}