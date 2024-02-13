use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_energy::entity::status_effect::StatusEffect;

#[derive(Debug)]
pub struct AttachSpecialEnergyToUnitIndexRequest {
    account_unique_id: i32,
    unit_card_index: i32,
    race_enum: RaceEnum,
    quantity: i32,
    status_effect_list: Vec<StatusEffect>
}

impl AttachSpecialEnergyToUnitIndexRequest {
    pub fn new(account_unique_id: i32, unit_card_index: i32, race_enum: RaceEnum, status_effect_list: Vec<StatusEffect>) -> Self {
        AttachSpecialEnergyToUnitIndexRequest {
            account_unique_id,
            unit_card_index,
            race_enum,
            quantity: 1,
            status_effect_list
        }
    }

    pub fn get_account_unique_id(&self) -> i32 {
        self.account_unique_id
    }

    pub fn get_unit_card_index(&self) -> i32 {
        self.unit_card_index
    }

    pub fn get_race_enum(&self) -> RaceEnum {
        self.race_enum
    }

    pub fn get_quantity(&self) -> i32 {
        self.quantity
    }

    pub fn get_status_effect_list(&self) -> &Vec<StatusEffect> {
        &self.status_effect_list
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
    use crate::game_card_energy::entity::effect::Effect;

    #[test]
    fn test_attach_special_energy_request() {
        // Replace with actual values for testing
        let account_unique_id = 123;
        let unit_card_index = 456;
        let race_enum = RaceEnum::Undead;  // Replace with the actual race enum variant
        let status_effect_list = vec![
            StatusEffect::new(Effect::Darkfire, 3, 10, -1),  // Replace with actual values
            StatusEffect::new(Effect::Freeze, 4, 15, -2),  // Replace with actual values
        ];

        let request = AttachSpecialEnergyToUnitIndexRequest::new(
            account_unique_id,
            unit_card_index,
            race_enum.clone(),
            status_effect_list.clone(),
        );

        assert_eq!(request.get_account_unique_id(), account_unique_id);
        assert_eq!(request.get_unit_card_index(), unit_card_index);
        assert_eq!(request.get_race_enum(), race_enum);
        assert_eq!(request.get_quantity(), 1);

        // Printing the request and status_effect_list for debugging
        println!("AttachSpecialEnergyToUnitIndexRequest: {:?}", request);
        println!("StatusEffectList: {:?}", request.status_effect_list());
    }
}
