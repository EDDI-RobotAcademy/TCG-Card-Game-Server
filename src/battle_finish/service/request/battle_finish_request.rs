use crate::game_winner_check::entity::finish_position_enum::FinishPositionEnum;

#[derive(Debug)]
pub struct BattleFinishRequest {
    session_id: String,
    finish_position: FinishPositionEnum
}

impl BattleFinishRequest {
    pub fn new(session_id: String, finish_position: FinishPositionEnum) -> Self {
        BattleFinishRequest {
            session_id: session_id.to_string(),
            finish_position,
        }
    }

    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }

    pub fn get_finish_position(&self) -> &FinishPositionEnum { &self.finish_position }
}