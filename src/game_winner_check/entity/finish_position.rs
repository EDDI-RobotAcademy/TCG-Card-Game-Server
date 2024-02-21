use crate::game_winner_check::entity::finish_position_enum::FinishPositionEnum;

pub struct GameFinishPosition {
    finish_position: FinishPositionEnum,
}

impl GameFinishPosition {
    pub fn new(finish_position: FinishPositionEnum) -> FinishPositionEnum {
        finish_position
    }

    pub fn get_finish_position(&self) -> &FinishPositionEnum {
        &self.finish_position
    }
}