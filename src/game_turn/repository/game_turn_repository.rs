use async_trait::async_trait;

pub trait GameTurnRepository {
    fn create_game_turn_object(&mut self, account_unique_id: i32) -> bool;
    fn next_game_turn(&mut self, account_unique_id: i32) -> i32;
    fn get_game_turn(&mut self, account_unique_id: i32) -> i32;
    fn remove_game_turn_hash_by_account_unique_id(&mut self, account_unique_id: i32) -> bool;

}