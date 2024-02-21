use crate::game_winner_check::entity::finish_position_enum::FinishPositionEnum;

pub trait GameWinnerCheckRepository {
    fn create_finish_position_object(&mut self, account_unique_id: i32, finish_position_enum: FinishPositionEnum) -> bool;
    fn add_finish_position_object(&mut self, account_unique_id: i32, finish_position_enum: FinishPositionEnum) -> bool;
    fn get_finish_position_enum(&mut self, account_unique_id: i32) -> Option<&FinishPositionEnum>;
}