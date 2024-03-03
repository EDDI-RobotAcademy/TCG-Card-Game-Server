use std::ops::Deref;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_energy::entity::status_effect::StatusEffect;
use crate::game_card_unit::entity::passive_status::PassiveStatus;
use crate::game_field_unit::entity::extra_effect::ExtraEffect;
use crate::game_field_unit::entity::extra_status_effect::ExtraStatusEffect;
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

    pub fn add_field_unit(&mut self, card: GameFieldUnitCard) -> i32 {
        self.game_field_unit_card_list.push(card);
        (self.game_field_unit_card_list.len() - 1) as i32
    }

    pub fn get_unit_card_id(&mut self, unit_card_index: usize) -> i32 {
        if let Some(unit) = self.game_field_unit_card_list.get_mut(unit_card_index) {
            return unit.get_card()
        }

        -1
    }

    pub fn get_attached_race_energy_count_of_field_unit(&mut self, unit_card_index: usize, race_enum: RaceEnum) -> i32 {
        if let Some(unit) = self.game_field_unit_card_list.get_mut(unit_card_index) {
            return *unit.get_attached_energy().get_energy_quantity(&RaceEnumValue::from(race_enum as i32)).unwrap_or(&-1)
        }

        -1
    }

    pub fn get_total_count_of_field_unit_energy(&mut self, unit_card_index: usize) -> i32 {
        if let Some(unit) = self.game_field_unit_card_list.get_mut(unit_card_index) {
            let mut attached_energy_map = unit.get_attached_energy().clone();
            return attached_energy_map.get_all_energy_count()
        }

        -1
    }

    pub fn get_all_field_unit_list(&self) -> &Vec<GameFieldUnitCard> {
        &self.game_field_unit_card_list
    }

    pub fn get_all_field_unit_list_mut(&mut self) -> &mut Vec<GameFieldUnitCard> {
        &mut self.game_field_unit_card_list
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
        }
    }

    // TODO: 불퇴전 특성에 대한 예외 처리 필요
    pub fn judge_death_of_unit(&mut self, unit_card_index: usize) -> i32 {
        if let Some(unit) = self.game_field_unit_card_list.get_mut(unit_card_index) {
            let is_alive = unit.is_alive();
            let current_unit_health_point = unit.get_unit_health_point().get_current_health_point();
            if is_alive && current_unit_health_point <= 0 {
                unit.set_is_alive(false);
                return unit.get_card()
            }
        }

        -1
    }

    pub fn check_unit_alive(&mut self, unit_card_index: usize) -> bool {
        if let Some(unit) = self.game_field_unit_card_list.get_mut(unit_card_index) {
            return unit.is_alive()
        }

        false
    }

    pub fn check_turn_action(&mut self, unit_card_index: usize) -> bool {
        self.game_field_unit_card_list.get_mut(unit_card_index).unwrap().get_turn_action()
    }
    pub fn execute_turn_action(&mut self, unit_card_index: usize) {
        self.game_field_unit_card_list.get_mut(unit_card_index).unwrap().set_turn_action(true);
    }
    pub fn reset_turn_action(&mut self, unit_card_index: usize) {
        self.game_field_unit_card_list.get_mut(unit_card_index).unwrap().set_turn_action(false);
    }

    pub fn add_special_energy_to_indexed_unit(&mut self, unit_card_index: usize, race_enum: RaceEnumValue, quantity: i32, status_effect_list: Vec<StatusEffect>) {
        if let Some(unit) = self.game_field_unit_card_list.get_mut(unit_card_index) {
            unit.attach_special_energy(race_enum, quantity, status_effect_list);
        }
    }

    pub fn impose_harmful_state_to_indexed_unit(&mut self, unit_card_index: usize, harmful_state: ExtraStatusEffect) {
        if let Some(unit) = self.game_field_unit_card_list.get_mut(unit_card_index) {
            unit.impose_harmful_state(harmful_state);
        }
    }

    pub fn impose_harmful_state_list_to_indexed_unit(&mut self, unit_card_index: usize, harmful_states: Vec<ExtraStatusEffect>) {
        if let Some(unit) = self.game_field_unit_card_list.get_mut(unit_card_index) {
            unit.impose_harmful_state_list(harmful_states);
        }
    }

    pub fn remove_harmful_status_of_indexed_unit(&mut self, unit_card_index: usize, extra_effect: &ExtraEffect) {
        if let Some(unit) = self.game_field_unit_card_list.get_mut(unit_card_index) {
            unit.remove_harmful_effect(extra_effect);
        }
    }

    // 해로운 효과 전역에 뿌리기
    pub fn impose_harmful_state_list_iteratively(&mut self, harmful_state_list: Vec<ExtraStatusEffect>) {
        for unit in &mut self.game_field_unit_card_list {
            for harmful_state in harmful_state_list.iter() {
                unit.impose_harmful_state(harmful_state.clone());
            }
        }
    }

    pub fn apply_status_effect_damage_iteratively(&mut self) {
        for unit in &mut self.game_field_unit_card_list {
            if unit.is_alive() {
                unit.apply_status_effect_damage();
            }
        }
    }

    pub fn impose_extra_effect_state_to_indexed_unit(&mut self, unit_card_index: usize, extra_state: ExtraStatusEffect) {
        if let Some(unit) = self.game_field_unit_card_list.get_mut(unit_card_index) {
            unit.impose_extra_effect_state(extra_state);
        }
    }

    pub fn remove_energy_from_indexed_unit(&mut self, unit_card_index: usize, race_enum: RaceEnumValue, quantity: i32) {
        if let Some(unit) = self.game_field_unit_card_list.get_mut(unit_card_index) {
            unit.detach_energy(race_enum, quantity);
        }
    }

    pub fn set_unit_deployed_round(&mut self, unit_card_index: usize, round: i32) {
        if let Some(unit) = self.game_field_unit_card_list.get_mut(unit_card_index) {
            unit.set_deployed_round(round);
        }
    }

    pub fn get_unit_deployed_round(&mut self, unit_card_index: usize) -> i32 {
        if let Some(unit) = self.game_field_unit_card_list.get_mut(unit_card_index) {
            return unit.get_deployed_round()
        }

        -1
    }

    pub fn set_unit_passive_status_list(&mut self, unit_card_index: usize, passive_status_list: Vec<PassiveStatus>) {
        if let Some(unit) = self.game_field_unit_card_list.get_mut(unit_card_index) {
            unit.set_passive_status_list(passive_status_list);
        }
    }

    pub fn reset_first_passive(&mut self, unit_card_index: usize, first_passive_default: bool) {
        self.game_field_unit_card_list.get_mut(unit_card_index).unwrap().set_has_first_passive_skill(first_passive_default);
    }
    pub fn reset_second_passive(&mut self, unit_card_index: usize, second_passive_default: bool) {
        self.game_field_unit_card_list.get_mut(unit_card_index).unwrap().set_has_second_passive_skill(second_passive_default);
    }
    pub fn reset_third_passive(&mut self, unit_card_index: usize, third_passive_default: bool) {
        self.game_field_unit_card_list.get_mut(unit_card_index).unwrap().set_has_third_passive_skill(third_passive_default);
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
    fn test_impose_harmful_state_to_indexed_unit() {
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

        game_field_unit_card_list.add_field_unit(game_field_unit_card1.clone());
        game_field_unit_card_list.add_field_unit(game_field_unit_card2.clone());

        let harmful_state = ExtraStatusEffect::new(ExtraEffect::DarkFire, 2, 5, 3);

        let unit_card_index = 1;
        println!("Before imposing harmful state to indexed unit: {:?}", game_field_unit_card_list);

        game_field_unit_card_list.impose_harmful_state_to_indexed_unit(unit_card_index, harmful_state.clone());

        println!("After imposing harmful state to indexed unit: {:?}", game_field_unit_card_list);

        let affected_unit = game_field_unit_card_list.find_unit_by_index(unit_card_index);
        println!("After imposing harmful state to indexed unit: {:?}", affected_unit);

        game_field_unit_card_list.apply_status_effect_damage_iteratively();
        let affected_unit = game_field_unit_card_list.find_unit_by_index(unit_card_index);
        println!("After imposing harmful state to indexed unit: {:?}", affected_unit);
    }

    #[test]
    fn test_impose_harmful_state_list_iteratively() {
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

        game_field_unit_card_list.add_field_unit(game_field_unit_card1.clone());
        game_field_unit_card_list.add_field_unit(game_field_unit_card2.clone());

        let harmful_state_list = vec![
            ExtraStatusEffect::new(ExtraEffect::DarkFire, 2, 5, 3),
            ExtraStatusEffect::new(ExtraEffect::Freeze, 3, 0, 2),
        ];

        println!("Before imposing harmful state list: {:?}", game_field_unit_card_list);

        game_field_unit_card_list.impose_harmful_state_list_iteratively(harmful_state_list);

        println!("After imposing harmful state list: {:?}", game_field_unit_card_list);
    }
}
