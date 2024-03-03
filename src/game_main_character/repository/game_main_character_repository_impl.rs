use std::sync::Arc;
use diesel::dsl::max;
use indexmap::IndexMap;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;

use crate::game_main_character::entity::game_main_character::GameMainCharacter;
use crate::game_main_character::entity::status_main_character::StatusMainCharacterEnum::Death;
use crate::game_main_character::repository::game_main_character_repository::GameMainCharacterRepository;

pub struct GameMainCharacterRepositoryImpl {
    game_main_character_map: IndexMap<i32, GameMainCharacter>,
}

impl GameMainCharacterRepositoryImpl {
    pub fn new() -> Self {
        GameMainCharacterRepositoryImpl {
            game_main_character_map: IndexMap::new(),
        }
    }

    pub fn get_game_main_character_map(&mut self) -> &mut IndexMap<i32, GameMainCharacter> {
        &mut self.game_main_character_map
    }

    pub fn get_instance() -> Arc<AsyncMutex<GameMainCharacterRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameMainCharacterRepositoryImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameMainCharacterRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

impl GameMainCharacterRepository for GameMainCharacterRepositoryImpl {
    fn create_game_main_character_object(&mut self, account_unique_id: i32) -> bool {
        println!("GameMainCharacterRepositoryImpl: create_game_main_character_object()");

        let new_game_main_character_map = GameMainCharacter::new(100);
        self.game_main_character_map.insert(account_unique_id, new_game_main_character_map);

        true
    }

    fn apply_damage_to_main_character(&mut self, account_unique_id: i32, damage: i32) -> bool {
        println!("GameMainCharacterRepositoryImpl: apply_damage_to_main_character()");

        if let Some(game_main_character) = self.game_main_character_map.get_mut(&account_unique_id) {
            game_main_character.decrease_health_point(damage);
        }

        true
    }

    fn get_health_point_of_main_character_by_account_unique_id(&mut self, account_unique_id: i32) -> i32 {
        println!("GameMainCharacterRepositoryImpl: get_health_point_of_main_character_by_account_unique_id()");

        let main_character_of_account_unique_id = self.game_main_character_map.get_mut(&account_unique_id).unwrap();
        let health_point_main_character = main_character_of_account_unique_id.get_health_point();

        return health_point_main_character;
    }

    fn set_main_character_status_death(&mut self, account_unique_id: i32) -> bool {
        println!("GameMainCharacterRepositoryImpl: set_main_character_status_death()");

        if let Some(game_main_character) = self.game_main_character_map.get_mut(&account_unique_id) {
            game_main_character.set_status(Death);
            return true
        }

        false
    }

    fn remove_game_main_character_hash_by_account_unique_id(&mut self, account_unique_id: i32) -> bool {
        if let Some(game_main_character) = self.game_main_character_map.get_mut(&account_unique_id) {
            self.game_main_character_map.remove(&account_unique_id);
            return true
        }
        return false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_main_character_object() {
        let mut game_main_character_repository = GameMainCharacterRepositoryImpl::new();
        let result = game_main_character_repository.create_game_main_character_object(1);

        assert!(result);

        println!("Test Output: {:?}", game_main_character_repository.get_game_main_character_map());

        let apply_damage = game_main_character_repository.apply_damage_to_main_character(1, 30);

        assert!(apply_damage);

        println!("After applying damage: {:?}", game_main_character_repository.get_game_main_character_map());
    }

    #[tokio::test]
    async fn test_get_instance() {
        let instance = GameMainCharacterRepositoryImpl::get_instance();

        let mut lock = instance.lock().await;

        assert!(Arc::strong_count(&instance) > 1);
        assert_eq!(lock.get_game_main_character_map().len(), 0);
    }
}
