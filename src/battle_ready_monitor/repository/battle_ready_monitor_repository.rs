use std::error::Error;
use async_trait::async_trait;
use crate::battle_ready_monitor::entity::battle_ready_status::BattleReadyStatus;

#[async_trait]
pub trait BattleReadyMonitorRepository {
    async fn save_battle_account_hash(&mut self, accounts_vector: Vec<i32>, state: BattleReadyStatus);
    async fn get_account_status(&mut self, account_unique_id: i32) -> Result<BattleReadyStatus, Box<dyn Error>>;
}