use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;

#[derive(Debug)]
pub struct DetachMultipleEnergyFromFieldUnitRequest {
    account_unique_id: i32,
    unit_card_index: i32,
    race_enum: RaceEnum,
    quantity: i32
}

impl DetachMultipleEnergyFromFieldUnitRequest {
    pub fn new(account_unique_id: i32, unit_card_index: i32, race_enum: RaceEnum, quantity: i32) -> Self {
        DetachMultipleEnergyFromFieldUnitRequest {
            account_unique_id,
            unit_card_index,
            race_enum,
            quantity
        }
    }

    pub fn get_account_unique_id(&self) -> i32 {
        self.account_unique_id
    }

    pub fn get_unit_card_index(&self) -> i32 {
        self.unit_card_index
    }

    pub fn get_race_enum(&self) -> &RaceEnum { &self.race_enum }

    pub fn get_quantity(&self) -> i32 {
        self.quantity
    }
}