use async_trait::async_trait;

pub trait GameRoundRepository {
    fn create_game_round_object(&mut self, account_unique_id: i32) -> bool;
    fn next_game_round_object(&mut self, account_unique_id: i32) -> i32;
    fn get_round_turn(&mut self, account_unique_id: i32) -> i32;
}