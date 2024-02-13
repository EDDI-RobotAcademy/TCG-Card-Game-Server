pub trait GameMainCharacterRepository {
    fn create_game_main_character_object(&mut self, account_unique_id: i32) -> bool;
    fn apply_damage_to_main_character(&mut self, account_unique_id: i32, damage: i32) -> bool;
}