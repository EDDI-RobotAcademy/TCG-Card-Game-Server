use std::sync::Arc;
use indexmap::IndexMap;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;

use crate::game_hand::entity::game_hand::GameHand;
use crate::game_hand::repository::game_hand_repository::GameHandRepository;

pub struct GameHandRepositoryImpl {
    game_hand_map: IndexMap<i32, GameHand>,
}

impl GameHandRepositoryImpl {
    pub fn new() -> Self {
        GameHandRepositoryImpl {
            game_hand_map: IndexMap::new(),
        }
    }

    pub(crate) fn get_game_hand_map(&mut self) -> &mut IndexMap<i32, GameHand> {
        &mut self.game_hand_map
    }

    pub fn get_instance() -> Arc<AsyncMutex<GameHandRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameHandRepositoryImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameHandRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

impl GameHandRepository for GameHandRepositoryImpl {
    fn create_game_hand_object(&mut self, account_unique_id: i32) -> bool {
        println!("GameHandRepositoryImpl: create_game_hand_object()");

        let new_game_hand_map = GameHand::new();
        self.game_hand_map.insert(account_unique_id, new_game_hand_map);

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_field_unit_object() {
        let mut game_hand_repository = GameHandRepositoryImpl::new();
        let result = game_hand_repository.create_game_hand_object(1);

        assert!(result);

        println!("Test Output: {:?}", game_hand_repository.get_game_hand_map());
    }

    #[tokio::test]
    async fn test_get_instance() {
        let instance = GameHandRepositoryImpl::get_instance();

        let mut lock = instance.lock().await;

        assert!(Arc::strong_count(&instance) > 1);
        assert_eq!(lock.get_game_hand_map().len(), 0);
    }
}
