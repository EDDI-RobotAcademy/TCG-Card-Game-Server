use std::error::Error;
use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;

use crate::battle_ready_monitor::entity::battle_ready_account_hash::BattleReadyAccountHash;
use crate::battle_ready_monitor::entity::battle_ready_status::BattleReadyStatus;
use crate::battle_ready_monitor::repository::battle_ready_monitor_repository::BattleReadyMonitorRepository;

pub struct BattleReadyMonitorRepositoryImpl {
    battle_ready_account_hash: BattleReadyAccountHash,
}

impl BattleReadyMonitorRepositoryImpl {
    pub fn new() -> Self {
        BattleReadyMonitorRepositoryImpl {
            battle_ready_account_hash: BattleReadyAccountHash::new(),
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<BattleReadyMonitorRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<BattleReadyMonitorRepositoryImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        BattleReadyMonitorRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl BattleReadyMonitorRepository for BattleReadyMonitorRepositoryImpl {
    async fn save_battle_account_hash(&mut self, accounts_vector: Vec<i32>, state: BattleReadyStatus) {
        println!("BattleReadyMonitorRepositoryImpl: save_battle_account_hash()");

        for account_id in accounts_vector {
            self.battle_ready_account_hash.set_user_ready_state(account_id, state);
        }
    }

    async fn get_account_status(&mut self, account_unique_id: i32) -> Result<BattleReadyStatus, Box<dyn Error>> {
        println!("BattleReadyMonitorRepositoryImpl: get_account_status()");

        self.battle_ready_account_hash.get_user_ready_state(account_unique_id).await.unwrap()
    }
}