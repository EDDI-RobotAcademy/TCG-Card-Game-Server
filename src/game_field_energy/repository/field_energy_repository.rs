pub trait FieldEnergyRepository {
    fn create_field_energy_object(&mut self, account_unique_id: i32) -> bool;
}