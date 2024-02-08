use crate::game_field_unit::entity::game_field_unit_card::GameFieldUnitCard;
use crate::game_field_unit::entity::race_enum_value::RaceEnumValue;

#[derive(Debug, Clone)]
pub struct GameFieldUnitCardList {
    game_field_unit_card_list: Vec<GameFieldUnitCard>,
}

impl GameFieldUnitCardList {
    pub fn new() -> GameFieldUnitCardList {
        GameFieldUnitCardList { game_field_unit_card_list: Vec::new() }
    }

    pub fn add_field_unit(&mut self, card: GameFieldUnitCard) {
        self.game_field_unit_card_list.push(card);
    }

    pub fn get_all_field_unit_list(&self) -> &Vec<GameFieldUnitCard> {
        &self.game_field_unit_card_list
    }

    pub fn find_unit_by_id(&self, unit_id: i32) -> Option<&GameFieldUnitCard> {
        self.game_field_unit_card_list.iter().find(|card| card.get_card() == unit_id)
    }

    pub fn add_energy_to_unit(&mut self, unit_id: i32, race: RaceEnumValue, quantity: i32) {
        if let Some(unit) = self.game_field_unit_card_list.iter_mut().find(|card| card.get_card() == unit_id) {
            unit.attach_energy(race, quantity);
        }
    }

    pub fn add_energy_to_indexed_unit(&mut self, unit_card_index: usize, race_enum: RaceEnumValue, quantity: i32) {
        if let Some(unit) = self.game_field_unit_card_list.get_mut(unit_card_index) {
            unit.attach_energy(race_enum, quantity);
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;
    use super::*;

    #[test]
    fn test_field_unit_list() {
        let mut game_field_unit_card_list = GameFieldUnitCardList::new();

        let game_field_unit_card1 = GameFieldUnitCard::new(3);
        let game_field_unit_card2 = GameFieldUnitCard::new(7);
        game_field_unit_card_list.add_field_unit(game_field_unit_card1);
        game_field_unit_card_list.add_field_unit(game_field_unit_card2);

        let field_unit_list = game_field_unit_card_list.get_all_field_unit_list();
        assert_eq!(field_unit_list.len(), 2);
        assert_eq!(field_unit_list[0].get_card(), 3);
        assert_eq!(field_unit_list[1].get_card(), 7);

        println!("{:?}", field_unit_list);
    }

    #[test]
    fn test_attach_energy_to_unit() {
        let mut game_field_unit_card_list = GameFieldUnitCardList::new();

        let game_field_unit_card1 = GameFieldUnitCard::new(3);
        let game_field_unit_card2 = GameFieldUnitCard::new(7);
        game_field_unit_card_list.add_field_unit(game_field_unit_card1);
        game_field_unit_card_list.add_field_unit(game_field_unit_card2);

        let field_unit_list = game_field_unit_card_list.get_all_field_unit_list();
        assert_eq!(field_unit_list.len(), 2);
        assert_eq!(field_unit_list[0].get_card(), 3);
        assert_eq!(field_unit_list[1].get_card(), 7);

        let mut cloned_list = game_field_unit_card_list.clone();
        cloned_list.add_energy_to_unit(3, RaceEnumValue::Undead, 3);

        println!("{:?}", cloned_list.get_all_field_unit_list());
    }

    #[test]
    fn test_find_unit_by_id() {
        let mut game_field_unit_card_list = GameFieldUnitCardList::new();

        let game_field_unit_card1 = GameFieldUnitCard::new(3);
        let game_field_unit_card2 = GameFieldUnitCard::new(7);
        game_field_unit_card_list.add_field_unit(game_field_unit_card1);
        game_field_unit_card_list.add_field_unit(game_field_unit_card2);

        let found_unit = game_field_unit_card_list.find_unit_by_id(3);
        println!("Found Unit: {:?}", found_unit);
        assert_eq!(found_unit.map(|unit| unit.get_card()), Some(3));

        let non_existent_unit = game_field_unit_card_list.find_unit_by_id(999);
        println!("Non-Existent Unit: {:?}", non_existent_unit);
        assert!(non_existent_unit.is_none());
    }

    #[test]
    fn test_ensure_vector_order() {
        let mut game_field_unit_card_list = GameFieldUnitCardList::new();
        let mut test_vector: Vec<i32> = Vec::new();

        let mut rng = rand::thread_rng();

        for _ in 0..7 {
            let random_id = rng.gen_range(1..=100);
            let game_field_unit_card = GameFieldUnitCard::new(random_id);
            println!("random_id: {}", random_id);
            game_field_unit_card_list.add_field_unit(game_field_unit_card);
            test_vector.push(random_id);
        }

        let field_unit_list = game_field_unit_card_list.get_all_field_unit_list();

        assert_eq!(field_unit_list.len(), 7);
        for (index, unit) in field_unit_list.iter().enumerate() {
            println!("Unit ID: {}, Expected ID: {}", unit.get_card(), index as i32 + 1);
            assert_eq!(unit.get_card(), test_vector[index]);
        }

        println!("{:?}", field_unit_list);
    }

    #[test]
    fn test_add_energy_to_indexed_unit() {
        let mut game_field_unit_card_list = GameFieldUnitCardList::new();
        let mut rng = rand::thread_rng();

        for _ in 0..7 {
            let random_id = rng.gen_range(1..=100);
            let game_field_unit_card = GameFieldUnitCard::new(random_id);
            game_field_unit_card_list.add_field_unit(game_field_unit_card);
        }

        let original_field_unit_list = game_field_unit_card_list.get_all_field_unit_list().clone();
        let index_to_add_energy = rng.gen_range(0..7);
        println!("index_to_add_energy: {}", index_to_add_energy);

        let race_enum = RaceEnumValue::Undead;
        let quantity = rng.gen_range(1..=10);
        game_field_unit_card_list.add_energy_to_indexed_unit(index_to_add_energy, race_enum, quantity);
        println!("quantity: {}", quantity);

        println!("Original Field Unit List: {:?}", original_field_unit_list);
        println!("Updated Field Unit List: {:?}", game_field_unit_card_list.get_all_field_unit_list());

        assert_eq!(
            game_field_unit_card_list.get_all_field_unit_list()[index_to_add_energy].get_attached_energy().get_energy_quantity(&race_enum),
            Some(quantity).as_ref()
        );
    }
}
