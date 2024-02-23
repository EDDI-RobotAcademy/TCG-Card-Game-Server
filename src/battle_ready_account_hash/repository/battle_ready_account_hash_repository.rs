use std::error::Error;
use async_trait::async_trait;
use crate::battle_ready_account_hash::entity::battle_ready_account_hash_status::BattleReadyAccountHashStatus;

#[async_trait]
pub trait BattleReadyAccountHashRepository {
    async fn save_battle_ready_account_list_hash(&mut self, accounts_vector: Vec<i32>, state: BattleReadyAccountHashStatus);
    async fn save_battle_ready_account_hash(&mut self, account_id: i32, state: BattleReadyAccountHashStatus);
    async fn get_battle_ready_account_hash_status(&mut self, account_unique_id: i32) -> BattleReadyAccountHashStatus;
    fn remove_battle_ready_account_hash_status_hash_by_account_unique_id(&mut self, account_unique_id: i32) -> bool;
}