#[derive(Debug)]
pub struct AttachMultipleEnergyToFieldUnitRequest {
    account_unique_id: i32,
    unit_number: i32,
    boost_race: i32,
    energy_count: i32,
}

impl AttachMultipleEnergyToFieldUnitRequest {
    pub fn new(account_unique_id: i32, unit_number: i32, boost_race: i32, energy_count: i32,) -> Self {
        AttachMultipleEnergyToFieldUnitRequest {
            account_unique_id,
            unit_number,
            boost_race,
            energy_count,
        }
    }

    pub fn get_account_unique_id(&self) -> i32 {
        self.account_unique_id
    }

    pub fn get_unit_number(&self) -> i32 {
        self.unit_number
    }

    pub fn get_boost_race(&self) -> i32 {
        self.boost_race
    }

    pub fn get_energy_count(&self) -> i32 {
        self.energy_count
    }
}
