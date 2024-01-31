pub trait GameMainCharacterRepository {
    fn create_game_main_character_object(&mut self, account_unique_id: i32) -> bool;
}