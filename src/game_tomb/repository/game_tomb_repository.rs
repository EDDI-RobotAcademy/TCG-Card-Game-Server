pub trait GameTombRepository {
    fn create_game_tomb_object(&mut self, account_unique_id: i32) -> bool;
    fn add_used_card_to_tomb(&mut self, account_unique_id: i32, used_card_id: i32) -> bool;
    fn add_dead_unit_to_tomb(&mut self, account_unique_id: i32, dead_unit_card_id: i32) -> bool;
}