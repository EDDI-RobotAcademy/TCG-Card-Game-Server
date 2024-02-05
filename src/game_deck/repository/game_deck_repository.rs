pub trait GameDeckRepository {
    fn create_game_deck_object(&mut self, account_unique_id: i32) -> bool;
    fn shuffle_game_deck(&mut self, account_unique_id: i32) -> bool;
    fn draw_deck_card(&mut self, account_unique_id: i32, draw_count: i32) -> Vec<i32>;
    fn add_cards_to_deck(&mut self, account_unique_id: i32, cards: Vec<i32>) -> bool;
}