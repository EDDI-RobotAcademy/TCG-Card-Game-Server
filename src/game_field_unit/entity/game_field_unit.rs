use crate::game_deck::entity::game_deck_card::DeckCard;
use crate::game_deck::entity::game_deck_card_list::DeckCardList;
use crate::game_field_unit::entity::field_unit::FieldUnit;
use crate::game_field_unit::entity::field_unit_list::FieldUnitList;

#[derive(Debug)]
pub struct GameFieldUnit {
    game_field_unit: FieldUnitList,
}

impl GameFieldUnit {
    pub fn new() -> GameFieldUnit {
        GameFieldUnit { game_field_unit: FieldUnitList::new() }
    }

    pub fn add_unit_to_game_field(&mut self, field_unit: FieldUnit) {
        self.game_field_unit.add_field_unit(field_unit);
    }

    pub fn get_all_unit_list_in_game_field(&self) -> &Vec<FieldUnit> {
        self.game_field_unit.get_all_field_unit_list()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_deck() {
        let mut game_field_unit = GameFieldUnit::new();

        let field_unit1 = FieldUnit::new(12);
        let field_unit2 = FieldUnit::new(34);

        game_field_unit.add_unit_to_game_field(field_unit1);
        game_field_unit.add_unit_to_game_field(field_unit2);

        let unit_list_in_game_field = game_field_unit.get_all_unit_list_in_game_field();

        assert_eq!(unit_list_in_game_field.len(), 2);
        assert_eq!(unit_list_in_game_field[0].get_card(), 12);
        assert_eq!(unit_list_in_game_field[1].get_card(), 34);

        println!("{:?}", unit_list_in_game_field);
    }
}
