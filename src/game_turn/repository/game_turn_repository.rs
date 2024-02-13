use async_trait::async_trait;

pub trait GameTurnRepository {
    fn create_game_turn_object(&mut self, account_unique_id: i32) -> bool;
    fn next_game_turn(&mut self, account_unique_id: i32) -> i32;
    fn decide_first_turn(&mut self, account_id: i32, player1_account_id:i32,player1_choice:String,
                                                     player2_account_id:i32,player2_choice:String,) -> (i32,bool,bool);
}