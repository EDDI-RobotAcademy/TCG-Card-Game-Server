use std::sync::Arc;
use indexmap::IndexMap;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;

use crate::game_field_unit::entity::game_field_unit::GameFieldUnit;
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
}
