pub trait GameFieldUnitRepository {
    fn create_game_field_unit_object(&mut self, account_unique_id: i32) -> bool;
    fn add_unit_to_game_field(&mut self, account_unique_id: i32, unit_card_number: i32) -> bool;
    fn attach_energy_to_game_field_unit(&mut self, account_unique_id: i32, unit_card_number: i32);
}