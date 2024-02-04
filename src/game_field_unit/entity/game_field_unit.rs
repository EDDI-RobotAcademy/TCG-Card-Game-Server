use crate::game_field_unit::entity::game_field_unit_card::GameFieldUnitCard;
use crate::game_field_unit::entity::game_field_unit_card_list::GameFieldUnitCardList;

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

    pub fn add_energy_to_unit(&mut self, unit_id: i32) {
        self.game_field_unit.add_energy_to_unit(unit_id);
    }

    pub fn get_all_unit_list_in_game_field(&self) -> &Vec<GameFieldUnitCard> {
        self.game_field_unit.get_all_field_unit_list()
    }
}

#[cfg(test)]
mod tests {
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

        let test_cases = [(3, 1), (7, 1)];

        for &(unit_id, expected_energy) in &test_cases {
            game_field_unit.add_energy_to_unit(unit_id);
            println!("After adding energy to unit {}: {:?}", unit_id, game_field_unit.get_all_unit_list_in_game_field());

            let unit_index = game_field_unit.get_all_unit_list_in_game_field()
                .iter()
                .position(|unit| unit.get_card() == unit_id)
                .expect("Unit not found in the list.");

            assert_eq!(game_field_unit.get_all_unit_list_in_game_field()[unit_index].get_attached_energy(), expected_energy);
        }
    }
}
