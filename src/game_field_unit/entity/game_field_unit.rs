use crate::game_card_energy::entity::status_effect::StatusEffect;
use crate::game_card_passive_skill::entity::summary_passive_skill_effect::SummaryPassiveSkillEffect;
use crate::game_field_unit::entity::extra_effect::ExtraEffect;
use crate::game_field_unit::entity::extra_status_effect::ExtraStatusEffect;
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

    pub fn add_unit_to_game_field(&mut self, field_unit: GameFieldUnitCard) -> i32 {
        self.game_field_unit.add_field_unit(field_unit)
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

    pub fn get_all_field_unit_list_mut(&mut self) -> &mut Vec<GameFieldUnitCard> {
        self.game_field_unit.get_all_field_unit_list_mut()
    }

    pub fn add_energy_to_indexed_unit(&mut self, unit_card_index: usize, race_enum: RaceEnumValue, quantity: i32) {
        self.game_field_unit.add_energy_to_indexed_unit(unit_card_index, race_enum, quantity);
    }

    pub fn increase_max_health_of_indexed_unit(&mut self, unit_card_index: usize, amount: i32) {
        self.game_field_unit.increase_max_health_of_indexed_unit(unit_card_index, amount);
    }

    pub fn find_unit_by_index(&self, index: usize) -> &GameFieldUnitCard {
        self.game_field_unit.find_unit_by_index(index)
    }

    pub fn apply_damage_to_indexed_unit(&mut self, unit_card_index: usize, damage: i32) {
        self.game_field_unit.apply_damage_to_indexed_unit(unit_card_index, damage);
    }

    pub fn apply_death_to_indexed_unit(&mut self, unit_card_index: usize) {
        self.game_field_unit.apply_death_to_indexed_unit(unit_card_index);
    }

    pub fn is_unit_alive(&mut self, unit_card_index: usize) {
        self.game_field_unit.is_unit_alive(unit_card_index);
    }
    pub fn check_unit_alive(&mut self, unit_card_index: usize) -> bool {
        return self.game_field_unit.check_unit_alive(unit_card_index)
    }

    pub fn add_special_energy_to_indexed_unit(&mut self, unit_card_index: usize, race_enum: RaceEnumValue, quantity: i32, status_effect_list: Vec<StatusEffect>) {
        self.game_field_unit.add_special_energy_to_indexed_unit(
            unit_card_index,
            race_enum,
            quantity,
            status_effect_list);
    }

    pub fn impose_harmful_state_to_indexed_unit(&mut self, unit_card_index: usize, harmful_state: ExtraStatusEffect) {
        self.game_field_unit.impose_harmful_state_to_indexed_unit(unit_card_index, harmful_state);
    }

    pub fn apply_status_effect_damage_iteratively(&mut self) {
        self.game_field_unit.apply_status_effect_damage_iteratively();
    }

    pub fn impose_extra_effect_state_to_indexed_unit(&mut self, unit_card_index: usize, extra_effect_state: SummaryPassiveSkillEffect) {

        let passive_skill_type = extra_effect_state.get_passive_skill_type();
        let passive_skill_type_number = passive_skill_type.clone() as i32;

        let effect = ExtraStatusEffect::new(
            ExtraEffect::from(passive_skill_type_number),
            -1,
            -1,
            -1
        );

        self.game_field_unit.impose_extra_effect_state_to_indexed_unit(unit_card_index, effect);
    }

    pub fn detach_energy_from_unit(&mut self, unit_card_index: usize, race_enum: RaceEnumValue, quantity: i32) {
        self.game_field_unit.remove_energy_from_indexed_unit(unit_card_index, race_enum, quantity);
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;
    use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;
    use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
    use crate::game_field_unit::entity::extra_effect::ExtraEffect;
    use super::*;

    #[test]
    fn test_add_game_field_unit() {
        let mut game_field_unit = GameFieldUnit::new();

        let field_unit1 = GameFieldUnitCard::new(
            12,
            RaceEnum::Chaos,
            GradeEnum::Hero,
            20,
            20,
            1,
            false,
            false,
            false,
            true);

        let field_unit2 = GameFieldUnitCard::new(
            34,
            RaceEnum::Angel,
            GradeEnum::Hero,
            20,
            20,
            1,
            false,
            false,
            false,
            true);

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

        let field_unit1 = GameFieldUnitCard::new(
            3,
            RaceEnum::Human,
            GradeEnum::Hero,
            20,
            20,
            1,
            false,
            false,
            false,
            true);

        let field_unit2 = GameFieldUnitCard::new(
            7,
            RaceEnum::Undead,
            GradeEnum::Hero,
            20,
            20,
            1,
            false,
            false,
            false,
            true);

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

        let field_unit1 = GameFieldUnitCard::new(
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

        let field_unit2 = GameFieldUnitCard::new(
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

        let field_unit1 = GameFieldUnitCard::new(
            3,
            RaceEnum::Machine,
            GradeEnum::Hero,
            20,
            20,
            1,
            false,
            false,
            false,
            true);

        let field_unit2 = GameFieldUnitCard::new(
            7,
            RaceEnum::Chaos,
            GradeEnum::Hero,
            20,
            20,
            1,
            false,
            false,
            false,
            true);

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

    #[test]
    fn test_increase_max_health_of_indexed_unit_in_game_field() {
        let mut game_field_unit = GameFieldUnit::new();
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

            game_field_unit.add_unit_to_game_field(game_field_unit_card);
        }

        let index_to_increase_max_health = rng.gen_range(0..3);
        let original_max_health = game_field_unit.get_all_unit_list_in_game_field()[index_to_increase_max_health].get_unit_health_point().get_max_health_point();
        let increase_amount = rng.gen_range(1..=10);

        game_field_unit.increase_max_health_of_indexed_unit(index_to_increase_max_health, increase_amount);

        let updated_max_health = game_field_unit.get_all_unit_list_in_game_field()[index_to_increase_max_health].get_unit_health_point().get_max_health_point();

        println!("Original Max Health: {}", original_max_health);
        println!("Updated Max Health: {}", updated_max_health);

        assert_eq!(updated_max_health, original_max_health + increase_amount);
    }

    #[test]
    fn test_find_unit_by_index() {
        let mut game_field_unit = GameFieldUnit::new();

        let field_unit1 = GameFieldUnitCard::new(
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

        let field_unit2 = GameFieldUnitCard::new(
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

        game_field_unit.add_unit_to_game_field(field_unit1);
        game_field_unit.add_unit_to_game_field(field_unit2);

        let unit_at_index_0 = game_field_unit.find_unit_by_index(0);
        println!("Unit at index 0: {:?}", unit_at_index_0);
        assert_eq!(unit_at_index_0.get_card(), 3);

        let unit_at_index_1 = game_field_unit.find_unit_by_index(1);
        println!("Unit at index 1: {:?}", unit_at_index_1);
        assert_eq!(unit_at_index_1.get_card(), 7);
    }

    #[test]
    fn test_apply_damage_to_indexed_unit_in_game_field() {
        let mut game_field_unit = GameFieldUnit::new();

        let field_unit1 = GameFieldUnitCard::new(
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

        let field_unit2 = GameFieldUnitCard::new(
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

        game_field_unit.add_unit_to_game_field(field_unit1.clone());
        game_field_unit.add_unit_to_game_field(field_unit2.clone());

        let original_health = game_field_unit.find_unit_by_index(0).get_unit_health_point().get_current_health_point();
        let damage_amount = 5;
        let index_to_apply_damage = 0;

        game_field_unit.apply_damage_to_indexed_unit(index_to_apply_damage, damage_amount);

        let updated_health = game_field_unit.find_unit_by_index(0).get_unit_health_point().get_current_health_point();

        println!("Original Health: {}", original_health);
        println!("Updated Health: {}", updated_health);

        assert_eq!(updated_health, original_health - damage_amount);
    }

    #[test]
    fn test_apply_status_effect_damage_iteratively() {
        let mut game_field_unit = GameFieldUnit::new();

        let unit1 = GameFieldUnitCard::new(1, RaceEnum::Human, GradeEnum::Hero, 20, 20, 1, false, false, false, true);
        let unit2 = GameFieldUnitCard::new(2, RaceEnum::Trent, GradeEnum::Hero, 20, 20, 1, false, false, false, true);
        game_field_unit.add_unit_to_game_field(unit1);
        game_field_unit.add_unit_to_game_field(unit2);

        let harmful_state = ExtraStatusEffect::new(ExtraEffect::Darkfire, 2, 5, 3);
        game_field_unit.game_field_unit.impose_harmful_state_list_iteratively(vec![harmful_state.clone()]);
        game_field_unit.apply_status_effect_damage_iteratively();

        for index in 0..game_field_unit.get_all_unit_list_in_game_field().len() {
            let updated_health = game_field_unit.find_unit_by_index(index).get_unit_health_point().get_current_health_point();
            let expected_damage = harmful_state.get_effect_damage();
            println!("Unit at index {}: Updated Health: {}, Expected Damage: {}", index, updated_health, expected_damage);
            assert_eq!(updated_health, 20 - expected_damage);
        }

        println!("game_field_unit: {:?}", game_field_unit);
    }
}
