pub trait GameFieldUnitRepository {
    fn create_game_field_unit_object(&mut self, account_unique_id: i32) -> bool;
}