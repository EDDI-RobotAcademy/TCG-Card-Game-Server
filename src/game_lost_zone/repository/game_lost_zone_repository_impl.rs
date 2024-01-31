use std::sync::Arc;
use indexmap::IndexMap;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;

use crate::game_lost_zone::entity::game_lost_zone::GameLostZone;
use crate::game_lost_zone::repository::game_lost_zone_repository::GameLostZoneRepository;

pub struct GameLostZoneRepositoryImpl {
    game_lost_zone_map: IndexMap<i32, GameLostZone>,
}

impl GameLostZoneRepositoryImpl {
    pub fn new() -> Self {
        GameLostZoneRepositoryImpl {
            game_lost_zone_map: IndexMap::new(),
        }
    }

    pub(crate) fn get_game_turn_map(&mut self) -> &mut IndexMap<i32, GameLostZone> {
        &mut self.game_lost_zone_map
    }

    pub fn get_instance() -> Arc<AsyncMutex<GameLostZoneRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameLostZoneRepositoryImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameLostZoneRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

impl GameLostZoneRepository for GameLostZoneRepositoryImpl {
    fn create_game_lost_zone_object(&mut self, account_unique_id: i32) -> bool {
        println!("GameLostZoneRepositoryImpl: create_game_lost_zone_object()");

        let new_game_lost_zone_map = GameLostZone::new();
        self.game_lost_zone_map.insert(account_unique_id, new_game_lost_zone_map);

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[tokio::test]
    async fn test_game_lost_zone_repository() {
        let mut repository = GameLostZoneRepositoryImpl::get_instance();
        let mut repository_guard = repository.lock().await;

        assert_eq!(repository_guard.create_game_lost_zone_object(1), true);

        let game_lost_zone_map = repository_guard.get_game_turn_map();
        assert_eq!(game_lost_zone_map.len(), 1);

        let lock_result = game_lost_zone_map.get(&1);
        assert!(lock_result.is_some());
    }
}
