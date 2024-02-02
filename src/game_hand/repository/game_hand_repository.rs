use std::println;

pub trait GameHandRepository {
    fn create_game_hand_object(&mut self, account_unique_id: i32) -> bool;
    fn add_card_list_to_hand(&mut self, account_unique_id: i32, card_list: Vec<i32>) -> bool;
}