use crate::game_card_energy::entity::status_effect::StatusEffect;
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

    pub fn increase_max_health_of_indexed_unit(&mut self, unit_card_index: usize, amount: i32) {
        if let Some(unit) = self.game_field_unit_card_list.get_mut(unit_card_index) {
            unit.increase_max_health(amount);
        }
    }

    pub fn find_unit_by_index(&self, index: usize) -> &GameFieldUnitCard {
        &self.game_field_unit_card_list[index]
    }

    pub fn apply_damage_to_indexed_unit(&mut self, unit_card_index: usize, damage: i32) {
        if let Some(unit) = self.game_field_unit_card_list.get_mut(unit_card_index) {
            unit.apply_damage(damage);
        }
    }

    pub fn apply_death_to_indexed_unit(&mut self, unit_card_index: usize) {
        if let Some(unit) = self.game_field_unit_card_list.get_mut(unit_card_index) {
            let unit_health_point = unit.get_mut_unit_health_point();
            unit_health_point.set_current_health_point(0);
            unit.set_is_alive(false);
        }
    }

    pub fn add_special_energy_to_indexed_unit(&mut self, unit_card_index: usize, race_enum: RaceEnumValue, quantity: i32, status_effect_list: Vec<StatusEffect>) {
        if let Some(unit) = self.game_field_unit_card_list.get_mut(unit_card_index) {
            unit.attach_special_energy(race_enum, quantity, status_effect_list);
        }
    }

    pub fn apply_status_effect_damage_iteratively(&mut self) {
        for unit in &mut self.game_field_unit_card_list {
            unit.apply_status_effect_damage();
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;
    use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;
    use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
    use crate::game_field_unit::entity::extra_effect::ExtraEffect;
    use crate::game_field_unit::entity::extra_status_effect::ExtraStatusEffect;
    use super::*;

    #[test]
    fn test_field_unit_list() {
        let mut game_field_unit_card_list = GameFieldUnitCardList::new();

        let game_field_unit_card1 = GameFieldUnitCard::new(
            3,
            RaceEnum::Angel,
            GradeEnum::Hero,
            20,
            20,
            1,
            false,
            false,
            false,
            true);

        let game_field_unit_card2 = GameFieldUnitCard::new(
            7,
            RaceEnum::Machine,
            GradeEnum::Hero,
            20,
            20,
            1,
            false,
            false,
            false,
            true);

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

        let game_field_unit_card1 = GameFieldUnitCard::new(
            3,
            RaceEnum::Angel,
            GradeEnum::Hero,
            20,
            20,
            1,
            false,
            false,
            false,
            true);

        let game_field_unit_card2 = GameFieldUnitCard::new(
            7,
            RaceEnum::Human,
            GradeEnum::Hero,
            20,
            20,
            1,
            false,
            false,
            false,
            true);

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

        let game_field_unit_card1 = GameFieldUnitCard::new(
            3,
            RaceEnum::Angel,
            GradeEnum::Hero,
            20,
            20,
            1,
            false,
            false,
            false,
            true);

        let game_field_unit_card2 = GameFieldUnitCard::new(
            7,
            RaceEnum::Trent,
            GradeEnum::Hero,
            20,
            20,
            1,
            false,
            false,
            false,
            true);

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
            let game_field_unit_card = GameFieldUnitCard::new(
                random_id,
                RaceEnum::Angel,
                GradeEnum::Hero,
                20,
                20,
                1,
                false,
                false,
                false,
                true);

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
            let game_field_unit_card = GameFieldUnitCard::new(
                random_id,
                RaceEnum::Chaos,
                GradeEnum::Hero,
                20,
                20,
                1,
                false,
                false,
                false,
                true);

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

    #[test]
    fn test_increase_max_health_of_indexed_unit() {
        let mut game_field_unit_card_list = GameFieldUnitCardList::new();
        let mut rng = rand::thread_rng();

        for _ in 0..3 {
            let random_id = rng.gen_range(1..=100);
            let game_field_unit_card = GameFieldUnitCard::new(
                random_id,
                RaceEnum::Chaos,
                GradeEnum::Hero,
                20,
                20,
                1,
                false,
                false,
                false,
                true);

            game_field_unit_card_list.add_field_unit(game_field_unit_card);
        }

        let index_to_increase_max_health = rng.gen_range(0..3);
        let original_max_health = game_field_unit_card_list.get_all_field_unit_list()[index_to_increase_max_health].get_unit_health_point().get_max_health_point();
        let increase_amount = rng.gen_range(1..=10);

        game_field_unit_card_list.increase_max_health_of_indexed_unit(index_to_increase_max_health, increase_amount);

        let updated_max_health = game_field_unit_card_list.get_all_field_unit_list()[index_to_increase_max_health].get_unit_health_point().get_max_health_point();

        println!("Original Max Health: {}", original_max_health);
        println!("Updated Max Health: {}", updated_max_health);

        assert_eq!(updated_max_health, original_max_health + increase_amount);
    }

    #[test]
    fn test_find_unit_by_index() {
        let mut game_field_unit_card_list = GameFieldUnitCardList::new();

        let game_field_unit_card1 = GameFieldUnitCard::new(
            3,
            RaceEnum::Angel,
            GradeEnum::Hero,
            20,
            20,
            1,
            false,
            false,
            false,
            true);

        let game_field_unit_card2 = GameFieldUnitCard::new(
            7,
            RaceEnum::Trent,
            GradeEnum::Hero,
            20,
            20,
            1,
            false,
            false,
            false,
            true);

        game_field_unit_card_list.add_field_unit(game_field_unit_card1);
        game_field_unit_card_list.add_field_unit(game_field_unit_card2);

        let unit_at_index_0 = game_field_unit_card_list.find_unit_by_index(0);
        println!("Unit at index 0: {:?}", unit_at_index_0);
        assert_eq!(unit_at_index_0.get_card(), 3);

        let unit_at_index_1 = game_field_unit_card_list.find_unit_by_index(1);
        println!("Unit at index 1: {:?}", unit_at_index_1);
        assert_eq!(unit_at_index_1.get_card(), 7);
    }

    #[test]
    fn test_apply_damage_to_indexed_unit() {
        let mut game_field_unit_card_list = GameFieldUnitCardList::new();

        let game_field_unit_card1 = GameFieldUnitCard::new(
            3,
            RaceEnum::Angel,
            GradeEnum::Hero,
            20,
            20,
            1,
            false,
            false,
            false,
            true
        );

        let game_field_unit_card2 = GameFieldUnitCard::new(
            7,
            RaceEnum::Trent,
            GradeEnum::Hero,
            20,
            20,
            1,
            false,
            false,
            false,
            true
        );

        game_field_unit_card_list.add_field_unit(game_field_unit_card1.clone());
        game_field_unit_card_list.add_field_unit(game_field_unit_card2.clone());

        let original_health = game_field_unit_card_list.find_unit_by_index(0).get_unit_health_point().get_current_health_point();
        let damage_amount = 5;
        let index_to_apply_damage = 0;

        game_field_unit_card_list.apply_damage_to_indexed_unit(index_to_apply_damage, damage_amount);

        let updated_health = game_field_unit_card_list.find_unit_by_index(0).get_unit_health_point().get_current_health_point();

        println!("Original Health: {}", original_health);
        println!("Updated Health: {}", updated_health);

        assert_eq!(updated_health, original_health - damage_amount);
    }

    #[test]
    fn test_apply_status_effect_damage_iteratively() {
        let mut game_field_unit_card_list = GameFieldUnitCardList::new();

        let game_field_unit_card1 = GameFieldUnitCard::new(
            3,
            RaceEnum::Angel,
            GradeEnum::Hero,
            20,
            20,
            1,
            false,
            false,
            false,
            true);

        let game_field_unit_card2 = GameFieldUnitCard::new(
            7,
            RaceEnum::Trent,
            GradeEnum::Hero,
            20,
            20,
            1,
            false,
            false,
            false,
            true);

        game_field_unit_card1.extra_status_effect_list.push(ExtraStatusEffect::new(
            ExtraEffect::Darkfire,
            2,
            5,
            3,
        ));

        game_field_unit_card_list.add_field_unit(game_field_unit_card1.clone());
        game_field_unit_card_list.add_field_unit(game_field_unit_card2.clone());

        println!("Before applying status effect damage: {:?}", game_field_unit_card_list);

        game_field_unit_card_list.apply_status_effect_damage_iteratively();

        println!("After applying status effect damage: {:?}", game_field_unit_card_list);
    }
}
