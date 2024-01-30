use std::error::Error;
use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::battle_room::entity::battle_room::BattleRoom;
use crate::battle_room::repository::battle_room_repository::BattleRoomRepository;

// TODO: 1 대 1 배틀 룸을 관리하는 것 (Domain 정리가 필요하다 파악됨)
pub struct BattleRoomRepositoryImpl {
    battle_room_count: Arc<AsyncMutex<i32>>,
    battle_room_list: Arc<AsyncMutex<Vec<BattleRoom>>>,
}

impl BattleRoomRepositoryImpl {
    pub fn new() -> Self {
        BattleRoomRepositoryImpl {
            battle_room_list: Arc::new(AsyncMutex::new(Vec::new())),
            battle_room_count: Arc::new(AsyncMutex::new(0))
        }
    }

    // pub fn add_battle_room(&self, battle_room: BattleRoom) {
    //     let mut list = self.battle_room_list.lock().await;
    //     list.push(battle_room);
    // }
    //
    // pub fn get_battle_rooms(&self) -> Vec<BattleRoom> {
    //     let list = self.battle_room_list.lock().unwrap();
    //     list.clone()
    // }
    //
    // pub fn get_battle_room_count(&self) -> usize {
    //     let list = self.battle_room_list.lock().unwrap();
    //     list.len()
    // }

    pub fn get_instance() -> Arc<AsyncMutex<BattleRoomRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<BattleRoomRepositoryImpl>> =
                Arc::new(AsyncMutex::new(BattleRoomRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl BattleRoomRepository for BattleRoomRepositoryImpl {
    async fn set_players_to_battle_room(&self, account_unique_id_list: Vec<i32>) -> Result<bool, Box<dyn Error>> {
        println!("BattleRoomRepositoryImpl: set_player_to_battle_room() -> {:?}", account_unique_id_list);

        let mut battle_room_list_guard = self.battle_room_list.lock().await;
        let mut battle_room_count = *self.battle_room_count.lock().await;

        let mut battle_room = BattleRoom::new();
        battle_room.add_player(account_unique_id_list[0]);
        battle_room.add_player(account_unique_id_list[1]);

        battle_room_list_guard.push(BattleRoom::new());
        battle_room_count += 1;

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::Duration;

    #[tokio::test]
    async fn test_set_player_to_battle_room() {
        let battle_room_repository = BattleRoomRepositoryImpl::new();

        let mut account_vector: Vec<i32> = Vec::new();
        account_vector.push(1);
        account_vector.push(2);

        let result = battle_room_repository.set_player_to_battle_room(account_vector).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);

        let mut other_account_vector: Vec<i32> = Vec::new();
        other_account_vector.push(3);
        other_account_vector.push(4);

        let result = battle_room_repository.set_player_to_battle_room(other_account_vector).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
    }
}
