use std::sync::Arc;
use indexmap::IndexMap;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;

use crate::game_field_unit::entity::game_field_unit::GameFieldUnit;
use crate::game_field_unit::entity::game_field_unit_card::GameFieldUnitCard;
use crate::game_field_unit::entity::race_enum_value::RaceEnumValue;
use crate::game_field_unit::repository::game_field_unit_repository::GameFieldUnitRepository;

pub struct GameFieldUnitRepositoryImpl {
    game_field_unit_map: IndexMap<i32, GameFieldUnit>,
}

impl GameFieldUnitRepositoryImpl {
    pub fn new() -> Self {
        GameFieldUnitRepositoryImpl {
            game_field_unit_map: IndexMap::new(),
        }
    }

    pub(crate) fn get_game_field_unit_map(&mut self) -> &mut IndexMap<i32, GameFieldUnit> {
        &mut self.game_field_unit_map
    }

    pub fn get_instance() -> Arc<AsyncMutex<GameFieldUnitRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameFieldUnitRepositoryImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameFieldUnitRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

impl GameFieldUnitRepository for GameFieldUnitRepositoryImpl {
    fn create_game_field_unit_object(&mut self, account_unique_id: i32) -> bool {
        println!("GameFieldUnitRepositoryImpl: create_game_field_unit_object()");

        let new_game_field_unit_map = GameFieldUnit::new();
        self.game_field_unit_map.insert(account_unique_id, new_game_field_unit_map);

        true
    }

    // TODO: unit 스펙 관련 사항이 필요함
    fn add_unit_to_game_field(&mut self,
                              account_unique_id: i32,
                              unit_card_number: i32,
                              unit_race: RaceEnum,
                              unit_grade: GradeEnum,
                              unit_attack_point: i32,
                              unit_health_point: i32,
                              unit_attack_required_energy: i32,
                              first_passive_skill: bool,
                              second_passive_skill: bool,
                              third_passive_skill: bool) -> bool {

        if let Some(game_field_unit) = self.game_field_unit_map.get_mut(&account_unique_id) {
            game_field_unit.add_unit_to_game_field(
                GameFieldUnitCard::new(
                    unit_card_number,
                    unit_race,
                    unit_grade,
                    unit_attack_point,
                    unit_health_point,
                    unit_attack_required_energy,
                    first_passive_skill,
                    second_passive_skill,
                    third_passive_skill));
            true
        } else {
            false
        }
    }

    // TODO: 수량 1개 (enum 관련 사항을 어떻게 처리 할 것인가 고찰이 필요함)
    fn attach_energy_to_game_field_unit(&mut self, account_unique_id: i32, unit_card_number: i32, race_enum: RaceEnum, quantity: i32) {
        if let Some(game_field_unit) = self.game_field_unit_map.get_mut(&account_unique_id) {
            let race = RaceEnumValue::from(race_enum as i32);
            game_field_unit.add_energy_to_unit(unit_card_number, race, quantity);
        }
    }

    // TODO: 여러 개 (enum 관련 사항을 어떻게 처리 할 것인가 고찰이 필요함)
    fn attach_multiple_energy_to_game_field_unit(&mut self, account_unique_id: i32, unit_card_number: i32, race_number: i32, quantity: i32) -> bool {
        if let Some(game_field_unit) = self.game_field_unit_map.get_mut(&account_unique_id) {
            let race = RaceEnumValue::from(race_number);
            game_field_unit.add_energy_to_unit(unit_card_number, race, quantity);

            return true
        }

        return false
    }

    fn find_unit_by_id(&self, account_unique_id: i32, unit_card_number: i32) -> Option<&GameFieldUnitCard> {
        if let Some(game_field_unit) = self.game_field_unit_map.get(&account_unique_id) {
            game_field_unit.find_unit_by_id(unit_card_number)
        } else {
            None
        }
    }

    fn attach_multiple_energy_to_indexed_unit(&mut self, account_unique_id: i32, unit_card_index: i32, race_enum: RaceEnum, quantity: i32) -> bool {
        println!("GameFieldUnitRepositoryImpl: attach_multiple_energy_to_indexed_unit()");

        if let Some(game_field_unit) = self.game_field_unit_map.get_mut(&account_unique_id) {
            game_field_unit.add_energy_to_indexed_unit(unit_card_index as usize, RaceEnumValue::from(race_enum as i32), quantity);
            return true
        }

        return false
    }

    fn increase_max_health_of_indexed_unit(&mut self, account_unique_id: i32, unit_card_index: usize, amount: i32) -> bool {
        if let Some(game_field_unit) = self.game_field_unit_map.get_mut(&account_unique_id) {
            game_field_unit.increase_max_health_of_indexed_unit(unit_card_index, amount);
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_field_unit_object() {
        let mut game_field_unit_repository = GameFieldUnitRepositoryImpl::new();
        let result = game_field_unit_repository.create_game_field_unit_object(1);

        assert!(result);

        println!("Test Output: {:?}", game_field_unit_repository.get_game_field_unit_map());
    }

    #[tokio::test]
    async fn test_get_instance() {
        let instance = GameFieldUnitRepositoryImpl::get_instance();

        let mut lock = instance.lock().await;

        assert!(Arc::strong_count(&instance) > 1);
        assert_eq!(lock.get_game_field_unit_map().len(), 0);
    }

    #[tokio::test]
    async fn test_add_unit_to_game_field() {
        let mut game_field_unit_repository = GameFieldUnitRepositoryImpl::new();
        game_field_unit_repository.create_game_field_unit_object(1);

        let unit_card_number = 42;
        let result = game_field_unit_repository.add_unit_to_game_field(
            1,
            unit_card_number,
            RaceEnum::Chaos,
            GradeEnum::Legend,
            35,
            30,
            2,
            false,
            false,
            false);

        assert!(result);

        println!("Test Output: {:?}", game_field_unit_repository.get_game_field_unit_map());
    }

    #[tokio::test]
    async fn test_attach_energy_to_game_field_unit() {
        let mut game_field_unit_repository = GameFieldUnitRepositoryImpl::new();
        game_field_unit_repository.create_game_field_unit_object(1);

        let unit_card_number = 42;
        game_field_unit_repository.add_unit_to_game_field(
            1,
            unit_card_number,
            RaceEnum::Human,
            GradeEnum::Legend,
            35,
            30,
            1,
            false,
            false,
            false);

        println!("Initial state: {:?}", game_field_unit_repository.get_game_field_unit_map());

        // let race = RaceEnumValue::Undead;
        let race_enum = RaceEnum::Undead;
        let race = RaceEnumValue::from(race_enum as i32);
        let quantity = 1;
        game_field_unit_repository.attach_energy_to_game_field_unit(1, unit_card_number, race_enum, quantity);
        println!("After attaching energy: {:?}", game_field_unit_repository.get_game_field_unit_map());

        let game_field_unit = game_field_unit_repository.get_game_field_unit_map().get(&1).unwrap();
        let attached_energy = game_field_unit.get_all_unit_list_in_game_field()[0].
            get_attached_energy().get_energy_quantity(&race);
        assert_eq!(attached_energy, Some(&quantity));
    }

    #[tokio::test]
    async fn test_find_unit_by_id() {
        let mut game_field_unit_repository = GameFieldUnitRepositoryImpl::new();
        game_field_unit_repository.create_game_field_unit_object(1);

        let unit_card_number = 42;
        let result = game_field_unit_repository.add_unit_to_game_field(
            1,
            unit_card_number,
            RaceEnum::Human,
            GradeEnum::Legend,
            35,
            30,
            2,
            false,
            false,
            false);

        let result = game_field_unit_repository.add_unit_to_game_field(
            1,
            6,
            RaceEnum::Undead,
            GradeEnum::Legend,
            35,
            30,
            2,
            false,
            false,
            false);

        let result = game_field_unit_repository.add_unit_to_game_field(
            1,
            2,
            RaceEnum::Trent,
            GradeEnum::Legend,
            35,
            30,
            2,
            false,
            false,
            false);

        let result = game_field_unit_repository.add_unit_to_game_field(
            1,
            13,
            RaceEnum::Chaos,
            GradeEnum::Legend,
            35,
            30,
            2,
            false,
            false,
            false);

        assert!(result);

        println!("Test Output: {:?}", game_field_unit_repository.get_game_field_unit_map());

        let found_unit = game_field_unit_repository.find_unit_by_id(1, 2);
        println!("Found Unit: {:?}", found_unit);
        assert!(found_unit.is_some());

        let found_unit = found_unit.unwrap();
        assert_eq!(found_unit.get_card(), 2);

        let found_unit = game_field_unit_repository.find_unit_by_id(1, 12312);
        println!("Found Unit: {:?}", found_unit);
        assert!(found_unit.is_none());
    }

    #[tokio::test]
    async fn test_increase_max_health_of_indexed_unit_in_repository() {
        let mut game_field_unit_repository = GameFieldUnitRepositoryImpl::new();
        game_field_unit_repository.create_game_field_unit_object(1);

        let unit_card_number = 42;
        game_field_unit_repository.add_unit_to_game_field(
            1,
            unit_card_number,
            RaceEnum::Chaos,
            GradeEnum::Legend,
            35,
            30,
            2,
            false,
            false,
            false);

        let current_max_health = game_field_unit_repository.get_game_field_unit_map()[&1]
            .get_all_unit_list_in_game_field()[0]
            .get_unit_health_point()
            .get_max_health_point();

        let increase_amount = 10;
        game_field_unit_repository.increase_max_health_of_indexed_unit(1, 0, increase_amount);

        let updated_max_health = game_field_unit_repository.get_game_field_unit_map()[&1]
            .get_all_unit_list_in_game_field()[0]
            .get_unit_health_point()
            .get_max_health_point();

        assert_eq!(updated_max_health, current_max_health + increase_amount);
    }
}
