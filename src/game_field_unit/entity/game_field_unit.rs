use crate::game_field_unit::entity::game_field_unit_card::GameFieldUnitCard;
use crate::game_field_unit::entity::game_field_unit_card_list::GameFieldUnitCardList;
use crate::game_field_unit::entity::race_enum_value::RaceEnumValue;

#[derive(Debug)]
pub struct GameFieldUnit {
    game_field_unit: GameFieldUnitCardList,
}

impl GameFieldUnit {
    pub fn new() -> GameFieldUnit {
        GameFieldUnit { game_field_unit: GameFieldUnitCardList::new() }
    }

    pub fn add_unit_to_game_field(&mut self, field_unit: GameFieldUnitCard) {
        self.game_field_unit.add_field_unit(field_unit);
    }

    pub fn add_energy_to_unit(&mut self, unit_id: i32, race: RaceEnumValue, quantity: i32) {
        self.game_field_unit.add_energy_to_unit(unit_id, race, quantity);
    }

    pub fn find_unit_by_id(&self, unit_id: i32) -> Option<&GameFieldUnitCard> {
        self.game_field_unit.find_unit_by_id(unit_id)
    }

    pub fn get_all_unit_list_in_game_field(&self) -> &Vec<GameFieldUnitCard> {
        self.game_field_unit.get_all_field_unit_list()
    }

    pub fn add_energy_to_indexed_unit(&mut self, unit_card_index: usize, race_enum: RaceEnumValue, quantity: i32) {
        self.game_field_unit.add_energy_to_indexed_unit(unit_card_index, race_enum, quantity);
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;
    use super::*;

    #[test]
    fn test_add_game_field_unit() {
        let mut game_field_unit = GameFieldUnit::new();

        let field_unit1 = GameFieldUnitCard::new(12);
        let field_unit2 = GameFieldUnitCard::new(34);

        game_field_unit.add_unit_to_game_field(field_unit1);
        game_field_unit.add_unit_to_game_field(field_unit2);

        let unit_list_in_game_field = game_field_unit.get_all_unit_list_in_game_field();

        assert_eq!(unit_list_in_game_field.len(), 2);
        assert_eq!(unit_list_in_game_field[0].get_card(), 12);
        assert_eq!(unit_list_in_game_field[1].get_card(), 34);

        println!("{:?}", unit_list_in_game_field);
    }

    #[test]
    fn test_add_energy_to_unit() {
        let mut game_field_unit = GameFieldUnit::new();

        let field_unit1 = GameFieldUnitCard::new(3);
        let field_unit2 = GameFieldUnitCard::new(7);

        game_field_unit.add_unit_to_game_field(field_unit1);
        game_field_unit.add_unit_to_game_field(field_unit2);
        println!("Initial state: {:?}", game_field_unit.get_all_unit_list_in_game_field());

        let test_cases = [(3, RaceEnumValue::Undead, 1), (7, RaceEnumValue::Human, 1)];

        for &(unit_id, race, quantity) in &test_cases {
            game_field_unit.add_energy_to_unit(unit_id, race, quantity);
            println!("After adding energy to unit {}: {:?}", unit_id, game_field_unit.get_all_unit_list_in_game_field());

            let unit_index = game_field_unit.get_all_unit_list_in_game_field()
                .iter()
                .position(|unit| unit.get_card() == unit_id)
                .expect("Unit not found in the list.");

            assert_eq!(game_field_unit.get_all_unit_list_in_game_field()[unit_index].get_attached_energy().get_energy_quantity(&race), Some(&quantity));
        }
    }

    #[test]
    fn test_find_unit_by_id() {
        let mut game_field_unit = GameFieldUnit::new();

        let field_unit1 = GameFieldUnitCard::new(3);
        let field_unit2 = GameFieldUnitCard::new(7);

        game_field_unit.add_unit_to_game_field(field_unit1);
        game_field_unit.add_unit_to_game_field(field_unit2);
        println!("Initial state: {:?}", game_field_unit.get_all_unit_list_in_game_field());

        let found_unit = game_field_unit.find_unit_by_id(3);
        println!("Found Unit: {:?}", found_unit);
        assert!(found_unit.is_some());
        assert_eq!(found_unit.unwrap().get_card(), 3);

        let found_unit = game_field_unit.find_unit_by_id(12345);
        println!("Found Unit: {:?}", found_unit);
        assert!(found_unit.is_none());
    }

    #[test]
    fn test_add_energy_to_indexed_unit() {
        let mut game_field_unit = GameFieldUnit::new();

        let field_unit1 = GameFieldUnitCard::new(3);
        let field_unit2 = GameFieldUnitCard::new(7);

        game_field_unit.add_unit_to_game_field(field_unit1);
        game_field_unit.add_unit_to_game_field(field_unit2);
        println!("Initial state: {:?}", game_field_unit.get_all_unit_list_in_game_field());

        let index_to_add_energy = rand::thread_rng().gen_range(0..2);
        println!("index_to_add_energy: {}", index_to_add_energy);

        let race_enum = RaceEnumValue::Undead;
        let quantity = rand::thread_rng().gen_range(1..=10);
        println!("quantity: {}", quantity);

        game_field_unit.add_energy_to_indexed_unit(index_to_add_energy, race_enum, quantity);
        println!("After adding energy to unit at index {}: {:?}", index_to_add_energy, game_field_unit.get_all_unit_list_in_game_field());

        assert_eq!(
            game_field_unit.get_all_unit_list_in_game_field()[index_to_add_energy].get_attached_energy().get_energy_quantity(&race_enum),
            Some(&quantity)
        );
    }
}
