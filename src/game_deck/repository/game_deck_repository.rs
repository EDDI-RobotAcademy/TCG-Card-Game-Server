pub trait GameDeckRepository {
    fn create_game_deck_object(&mut self, account_unique_id: i32) -> bool;
    fn shuffle_game_deck(&mut self, account_unique_id: i32) -> bool;
    fn draw_deck_card(&mut self, account_unique_id: i32, draw_count: i32) -> Vec<i32>;
    fn add_cards_to_deck(&mut self, account_unique_id: i32, cards: Vec<i32>) -> bool;
    fn find_by_card_id_with_count(&mut self, account_id: i32, need_to_find_card_id: i32, energy_count: i32) -> Vec<i32>;
    fn find_deck_card_id_with_index(&mut self, account_unique_id: i32, deck_card_index: i32) -> i32;
    fn get_deck_card_by_index(&mut self, account_unique_id: i32, deck_card_index: i32) -> i32;
    fn get_remain_deck_card_count(&self, account_id: i32) -> i32;
    fn remove_game_deck_hash_by_account_unique_id(&mut self, account_unique_id: i32) -> bool;
}
