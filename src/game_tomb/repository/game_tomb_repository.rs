pub trait GameTombRepository {
    fn create_game_tomb_object(&mut self, account_unique_id: i32) -> bool;
}