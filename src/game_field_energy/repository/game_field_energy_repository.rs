pub trait GameFieldEnergyRepository {
    fn create_field_energy_object(&mut self, account_unique_id: i32) -> bool;
    fn add_field_energy_with_amount(&mut self, account_unique_id: i32, amount: i32) -> bool;
    fn remove_field_energy_with_amount(&mut self, account_unique_id: i32, amount: i32) -> bool;
    fn check_field_energy_enough_to_use(&mut self, account_unique_id: i32, will_be_used_field_energy_count: i32) -> bool;
    fn remove_game_field_energy_hash_by_account_unique_id(&mut self, account_unique_id: i32) -> bool;
}