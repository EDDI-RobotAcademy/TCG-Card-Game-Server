use std::collections::HashMap;
use std::error::Error;
use std::ops::Deref;
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
    battle_room_account_hash: Arc<AsyncMutex<HashMap<i32, i32>>>
}

impl BattleRoomRepositoryImpl {
    pub fn new() -> Self {
        BattleRoomRepositoryImpl {
            battle_room_list: Arc::new(AsyncMutex::new(Vec::new())),
            battle_room_count: Arc::new(AsyncMutex::new(0)),
            battle_room_account_hash: Arc::new(AsyncMutex::new(HashMap::new()))
        }
    }

    pub fn get_battle_room_account_hash(&self) -> &Arc<AsyncMutex<HashMap<i32, i32>>> {
        &self.battle_room_account_hash
    }

    pub async fn get_battle_room_count(&self) -> usize {
        let list = self.battle_room_list.lock().await;
        list.len()
    }

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
        let mut battle_room_count_guard = self.battle_room_count.lock().await;

        let mut battle_room = BattleRoom::new();
        battle_room.add_player(account_unique_id_list[0]);
        battle_room.add_player(account_unique_id_list[1]);

        // battle_room_list_guard.push(BattleRoom::new());
        battle_room_list_guard.push(battle_room);

        let mut battle_room_account_hash_guard = self.battle_room_account_hash.lock().await;
        battle_room_account_hash_guard.insert(account_unique_id_list[0], *battle_room_count_guard);
        battle_room_account_hash_guard.insert(account_unique_id_list[1], *battle_room_count_guard);

        *battle_room_count_guard += 1;
        println!("battle_room_count: {}", *battle_room_count_guard);

        Ok(true)
    }

    async fn remove_battle_room_player(&mut self, account_unique_id: i32) -> bool {
        println!("BattleRoomRepositoryImpl: remove_battle_room_player()");
        let room_number = self.what_is_the_room_number(account_unique_id).await.unwrap();
        let mut battle_room_list_guard = self.battle_room_list.lock().await;
        let mut battle_room = battle_room_list_guard.get_mut(room_number as usize).unwrap();
        battle_room.remove_player_for_finish(account_unique_id);
        drop(battle_room_list_guard);
        
        let mut battle_room_account_hash_guard = self.battle_room_account_hash.lock().await;
        battle_room_account_hash_guard.remove(&account_unique_id);
        drop(battle_room_account_hash_guard);

        return false;
    }

    async fn what_is_the_room_number(&self, account_unique_id: i32) -> Option<i32> {
        println!("BattleRoomRepositoryImpl: what_is_the_room_number()");

        let battle_room_account_hash_guard = self.battle_room_account_hash.lock().await;
        let result = match battle_room_account_hash_guard.get(&account_unique_id) {
            Some(&room_number) => Some(room_number),
            None => None,
        };
        drop(battle_room_account_hash_guard);

        return result
    }

    async fn find_opponent_unique_id(&self, account_unique_id: i32) -> Option<i32> {
        let battle_room_account_hash_guard = self.battle_room_account_hash.lock().await;

        match battle_room_account_hash_guard.get(&account_unique_id) {
            Some(&room_number) => {
                println!("room_number: {}", room_number);

                for (other_account_id, other_room_number) in battle_room_account_hash_guard.iter() {
                    println!("other_account_id: {}, other_room_number: {}", other_account_id, other_room_number);

                    if *other_room_number == room_number && *other_account_id != account_unique_id {
                        return Some(*other_account_id);
                    }
                }
                None
            }
            None => {
                println!("Account not in any room");
                None
            }
        }
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

        let result = battle_room_repository.set_players_to_battle_room(account_vector).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);

        let mut other_account_vector: Vec<i32> = Vec::new();
        other_account_vector.push(3);
        other_account_vector.push(4);

        let result = battle_room_repository.set_players_to_battle_room(other_account_vector).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
    }

    #[tokio::test]
    async fn test_find_opponent_unique_id() {
        let battle_room_repository = BattleRoomRepositoryImpl::get_instance();
        let battle_room_repository_guard = battle_room_repository.lock().await;

        let account1 = 1;
        let account2 = 2;
        let account_vector: Vec<i32> = vec![account1, account2];

        battle_room_repository_guard.set_players_to_battle_room(account_vector.clone()).await.unwrap();
        let account_hash = battle_room_repository_guard.get_battle_room_account_hash().lock().await;
        println!("account_hash: {:?}", account_hash);
        drop(account_hash);

        let result_account1 = battle_room_repository_guard.find_opponent_unique_id(account1).await;
        println!("result_account1: {:?}", result_account1);
        assert_eq!(result_account1, Some(account2));

        let result_account2 = battle_room_repository_guard.find_opponent_unique_id(account2).await;
        assert_eq!(result_account2, Some(account1));

        let result_unknown_account = battle_room_repository_guard.find_opponent_unique_id(42).await;
        assert_eq!(result_unknown_account, None);

        let account3 = 3;
        let account4 = 7;
        let account_vector2: Vec<i32> = vec![account3, account4];

        battle_room_repository_guard.set_players_to_battle_room(account_vector2.clone()).await.unwrap();
        let account_hash = battle_room_repository_guard.get_battle_room_account_hash().lock().await;
        println!("account_hash: {:?}", account_hash);
        drop(account_hash);

        let result_account3 = battle_room_repository_guard.find_opponent_unique_id(account3).await;
        println!("result_account3: {:?}", result_account3);
        assert_eq!(result_account3, Some(account4));

        let result_account4 = battle_room_repository_guard.find_opponent_unique_id(account4).await;
        assert_eq!(result_account4, Some(account3));
    }

    #[tokio::test]
    async fn test_remove_player_to_battle_room() {

        let battle_room_repository = BattleRoomRepositoryImpl::get_instance();
        let mut battle_room_repository_guard = battle_room_repository.lock().await;

        let mut account_vector: Vec<i32> = Vec::new();
        account_vector.push(1);
        account_vector.push(2);
        println!("account_vector: {:?}", account_vector);
        let account_unique_id = 1;

        // battle_room 셋팅
        let result = battle_room_repository_guard.set_players_to_battle_room(account_vector).await;

        let room_number = battle_room_repository_guard.what_is_the_room_number(account_unique_id).await.unwrap();
        println!("room_number: {:?}", room_number);

        let battle_room_list_guard = battle_room_repository_guard.battle_room_list.lock().await;
        let battle_room = battle_room_list_guard.get(room_number as usize).unwrap();
        println!("battle_room_before_test: {:?}", battle_room);
        drop(battle_room_list_guard);

        let battle_room_account_guard = battle_room_repository_guard.battle_room_account_hash.lock().await;
        println!("battle_room_account_hash_before_test: {:?}", battle_room_account_guard);
        drop(battle_room_account_guard);

        // account_unique_id 관련하여 battle_room_list 안의 BattleRoom 정보 삭제, battle_room_account_hash 안의 계정 정보 삭제
        let result_remove = battle_room_repository_guard.remove_battle_room_player(account_unique_id).await;

        let battle_room_list_guard = battle_room_repository_guard.battle_room_list.lock().await;
        let battle_room = battle_room_list_guard.get(room_number as usize).unwrap();
        println!("battle_room_after_test: {:?}", battle_room_list_guard);
        drop(battle_room_list_guard);

        let battle_room_account_hash_guard = battle_room_repository_guard.battle_room_account_hash.lock().await;
        println!("battle_room_account_hash_after_test: {:?}", battle_room_account_hash_guard);
        drop(battle_room_account_hash_guard);

    }
}
