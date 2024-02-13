pub trait GameFieldEnergyRepository {
    fn create_field_energy_object(&mut self, account_unique_id: i32) -> bool;
    fn add_field_energy_with_amount(&mut self, account_unique_id: i32, amount: i32) -> bool;
    fn remove_field_energy_with_amount(&mut self, account_unique_id: i32, amount: i32) -> bool;
}