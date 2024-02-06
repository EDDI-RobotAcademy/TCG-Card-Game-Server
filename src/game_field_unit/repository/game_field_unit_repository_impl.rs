use std::sync::Arc;
use indexmap::IndexMap;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;

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

    fn add_unit_to_game_field(&mut self, account_unique_id: i32, unit_card_number: i32) -> bool {
        if let Some(game_field_unit) = self.game_field_unit_map.get_mut(&account_unique_id) {
            game_field_unit.add_unit_to_game_field(GameFieldUnitCard::new(unit_card_number));
            true
        } else {
            false
        }
    }

    // TODO: 수량 1개
    fn attach_energy_to_game_field_unit(&mut self, account_unique_id: i32, unit_card_number: i32, race: RaceEnumValue, quantity: i32) {
        if let Some(game_field_unit) = self.game_field_unit_map.get_mut(&account_unique_id) {
            game_field_unit.add_energy_to_unit(unit_card_number, race, quantity);
        }
    }

    // TODO: 여러 개
    fn attach_multiple_energy_to_game_field_unit(&mut self, account_unique_id: i32, unit_card_number: i32, race_number: i32, quantity: i32) {
        if let Some(game_field_unit) = self.game_field_unit_map.get_mut(&account_unique_id) {
            let race = RaceEnumValue::from(race_number);
            game_field_unit.add_energy_to_unit(unit_card_number, race, quantity);
        }
    }

    fn find_unit_by_id(&self, account_unique_id: i32, unit_card_number: i32) -> Option<&GameFieldUnitCard> {
        if let Some(game_field_unit) = self.game_field_unit_map.get(&account_unique_id) {
            game_field_unit.find_unit_by_id(unit_card_number)
        } else {
            None
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
        let result = game_field_unit_repository.add_unit_to_game_field(1, unit_card_number);

        assert!(result);

        println!("Test Output: {:?}", game_field_unit_repository.get_game_field_unit_map());
    }

    #[tokio::test]
    async fn test_attach_energy_to_game_field_unit() {
        let mut game_field_unit_repository = GameFieldUnitRepositoryImpl::new();
        game_field_unit_repository.create_game_field_unit_object(1);

        let unit_card_number = 42;
        game_field_unit_repository.add_unit_to_game_field(1, unit_card_number);
        println!("Initial state: {:?}", game_field_unit_repository.get_game_field_unit_map());

        let race = RaceEnumValue::Undead;
        let quantity = 1;
        game_field_unit_repository.attach_energy_to_game_field_unit(1, unit_card_number, race, quantity);
        println!("After attaching energy: {:?}", game_field_unit_repository.get_game_field_unit_map());

        let game_field_unit = game_field_unit_repository.get_game_field_unit_map().get(&1).unwrap();
        let attached_energy = game_field_unit.get_all_unit_list_in_game_field()[0].get_attached_energy().get_energy_quantity(&race);
        assert_eq!(attached_energy, Some(&quantity));
    }

    #[tokio::test]
    async fn test_find_unit_by_id() {
        let mut game_field_unit_repository = GameFieldUnitRepositoryImpl::new();
        game_field_unit_repository.create_game_field_unit_object(1);

        let unit_card_number = 42;
        let result = game_field_unit_repository.add_unit_to_game_field(1, unit_card_number);
        let result = game_field_unit_repository.add_unit_to_game_field(1, 6);
        let result = game_field_unit_repository.add_unit_to_game_field(1, 2);
        let result = game_field_unit_repository.add_unit_to_game_field(1, 13);
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
}
