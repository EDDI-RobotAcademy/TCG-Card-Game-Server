#[derive(Debug)]
pub struct IsUnitBasicAttackPossibleRequest {
    account_unique_id: i32,
    field_unit_index: i32,
    basic_attack_required_energy_count: i32,
}

impl IsUnitBasicAttackPossibleRequest {
    pub fn new(account_unique_id: i32,
               field_unit_index: i32,
               basic_attack_required_energy_count: i32,) -> Self {
        IsUnitBasicAttackPossibleRequest {
            account_unique_id,
            field_unit_index,
            basic_attack_required_energy_count
        }
    }

    pub fn get_account_unique_id(&self) -> i32 { self.account_unique_id }

    pub fn get_field_unit_index(&self) -> i32 { self.field_unit_index }

    pub fn get_basic_attack_required_energy_count(&self) -> i32 { self.basic_attack_required_energy_count }
}