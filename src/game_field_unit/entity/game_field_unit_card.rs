use crate::game_field_unit::entity::attached_energy_map::AttachedEnergyMap;
use crate::game_field_unit::entity::race_enum_value::RaceEnumValue;

#[derive(Debug, Clone)]
pub struct GameFieldUnitCard {
    field_unit_card: i32,
    attached_energy_map: AttachedEnergyMap,
}

impl GameFieldUnitCard {
    pub fn new(field_unit_card: i32) -> GameFieldUnitCard {
        GameFieldUnitCard {
            field_unit_card,
            attached_energy_map: AttachedEnergyMap::new(),
        }
    }

    pub fn get_card(&self) -> i32 {
        self.field_unit_card
    }

    pub fn get_attached_energy(&self) -> &AttachedEnergyMap {
        &self.attached_energy_map
    }

    pub fn attach_energy(&mut self, race: RaceEnumValue, quantity: i32) {
        self.attached_energy_map.add_energy(race, quantity);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_field_unit_card_creation() {
        let game_field_unit_card = GameFieldUnitCard::new(5);
        assert_eq!(game_field_unit_card.get_card(), 5);
        println!("Test passed: FieldUnit creation and getter");
    }

    #[test]
    fn test_attach_energy() {
        let mut game_field_unit_card = GameFieldUnitCard::new(5);
        assert_eq!(game_field_unit_card.get_card(), 5);
        assert_eq!(game_field_unit_card.get_attached_energy().get_all_energy().len(), 0);

        game_field_unit_card.attach_energy(RaceEnumValue::Undead, 3);
        assert_eq!(game_field_unit_card.get_attached_energy().get_all_energy().len(), 1);
        assert_eq!(*game_field_unit_card.get_attached_energy().get_energy_quantity(&RaceEnumValue::Undead).unwrap(), 3);

        game_field_unit_card.attach_energy(RaceEnumValue::Human, 5);
        assert_eq!(game_field_unit_card.get_attached_energy().get_all_energy().len(), 2);
        assert_eq!(*game_field_unit_card.get_attached_energy().get_energy_quantity(&RaceEnumValue::Human).unwrap(), 5);

        println!("{:?}", game_field_unit_card);

        println!("Test passed: FieldUnit creation, getter, attach_energy, and print_state");
    }
}
