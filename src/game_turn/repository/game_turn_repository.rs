use async_trait::async_trait;

use crate::game_turn::service::response::decide_first_turn_response::DecideFirstTurnResponse;
pub trait GameTurnRepository {
    fn create_game_turn_object(&mut self, account_unique_id: i32) -> bool;
    fn next_game_turn(&mut self, account_unique_id: i32) -> i32;
    fn decide_first_turn(&mut self, account_unique_id1: i32, choice1:String,
                                    account_unique_id2: i32, choice2:String) -> DecideFirstTurnResponse;
}