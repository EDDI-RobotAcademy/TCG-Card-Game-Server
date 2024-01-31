use async_trait::async_trait;

pub trait GameTurnRepository {
    fn create_game_turn_object(&mut self, account_unique_id: i32) -> bool;
}