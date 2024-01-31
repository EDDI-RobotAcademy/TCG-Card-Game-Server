pub trait GameHandRepository {
    fn create_game_hand_object(&mut self, account_unique_id: i32) -> bool;
}