use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;

#[derive(Debug)]
pub struct GetCurrentAttachedEnergyOfFieldUnitByIndexRequest {
    account_unique_id: i32,
    field_unit_index: i32,
    energy_race: RaceEnum,
}

impl GetCurrentAttachedEnergyOfFieldUnitByIndexRequest {
    pub fn new(account_unique_id: i32, field_unit_index: i32, energy_race: RaceEnum,) -> Self {
        GetCurrentAttachedEnergyOfFieldUnitByIndexRequest {
            account_unique_id,
            field_unit_index,
            energy_race,
        }
    }
    pub fn get_account_unique_id(&self) -> i32 {
        self.account_unique_id
    }

    pub fn get_field_unit_index(&self) -> i32 {
        self.field_unit_index
    }

    pub fn get_energy_race(&self) -> &RaceEnum { &self.energy_race }
}