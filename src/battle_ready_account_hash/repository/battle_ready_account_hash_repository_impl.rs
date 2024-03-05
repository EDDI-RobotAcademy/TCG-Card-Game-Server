use std::error::Error;
use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::battle_ready_account_hash::entity::battle_ready_account_hash::BattleReadyAccountHash;
use crate::battle_ready_account_hash::entity::battle_ready_account_hash_status::BattleReadyAccountHashStatus;
use crate::battle_ready_account_hash::repository::battle_ready_account_hash_repository::BattleReadyAccountHashRepository;

pub struct BattleReadyAccountHashRepositoryImpl {
    battle_ready_account_hash_status: BattleReadyAccountHash,
}

impl BattleReadyAccountHashRepositoryImpl {
    pub fn new() -> Self {
        BattleReadyAccountHashRepositoryImpl {
            battle_ready_account_hash_status: BattleReadyAccountHash::new(),
        }
    }

    pub fn get_battle_ready_account_hash(&self) -> &BattleReadyAccountHash {
        &self.battle_ready_account_hash_status
    }

    pub fn get_instance() -> Arc<AsyncMutex<BattleReadyAccountHashRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<BattleReadyAccountHashRepositoryImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        BattleReadyAccountHashRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl BattleReadyAccountHashRepository for BattleReadyAccountHashRepositoryImpl {
    async fn save_battle_ready_account_list_hash(&mut self, accounts_vector: Vec<i32>, state: BattleReadyAccountHashStatus) {
        println!("BattleReadyAccountHashRepositoryImpl: save_battle_account_list_hash()");

        for account_id in accounts_vector {
            self.battle_ready_account_hash_status.set_user_ready_state(account_id, state);
        }
    }

    async fn save_battle_ready_account_hash(&mut self, account_id: i32, state: BattleReadyAccountHashStatus) {
        println!("BattleReadyAccountHashRepositoryImpl: save_battle_account_hash()");

        self.battle_ready_account_hash_status.set_user_ready_state(account_id, state);
    }

    async fn get_battle_ready_account_hash_status(&mut self, account_unique_id: i32) -> BattleReadyAccountHashStatus {
        println!("BattleReadyAccountHashRepositoryImpl: get_account_status()");

        let battle_ready_status = self.battle_ready_account_hash_status.get_user_ready_state(account_unique_id);
        return *battle_ready_status.unwrap()
    }

    fn remove_battle_ready_account_hash_status_hash_by_account_unique_id(&mut self, account_unique_id: i32) -> bool {
        println!("BattleReadyAccountHashRepositoryImpl: remove_battle_ready_account_hash_status_hash()");
        if let Some(_) = self.battle_ready_account_hash_status.get_user_ready_state(account_unique_id) {
            self.battle_ready_account_hash_status.remove_battle_read_account_hash_status(account_unique_id);
            return true
        }
        return false
    }
}
