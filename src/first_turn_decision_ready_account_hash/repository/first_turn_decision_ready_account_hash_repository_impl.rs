use std::error::Error;
use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::first_turn_decision_ready_account_hash::entity::first_turn_decision_ready_account_hash::FirstTurnDecisionReadyAccountHash;
use crate::first_turn_decision_ready_account_hash::entity::first_turn_decision_ready_account_hash_status::FirstTurnDecisionReadyAccountHashStatus;
use crate::first_turn_decision_ready_account_hash::repository::first_turn_decision_ready_account_hash_repository::FirstTurnDecisionReadyAccountHashRepository;

pub struct FirstTurnDecisionReadyAccountHashRepositoryImpl {
    first_turn_decision_ready_account_hash_status: FirstTurnDecisionReadyAccountHash,
}

impl FirstTurnDecisionReadyAccountHashRepositoryImpl {
    pub fn new() -> Self {
        FirstTurnDecisionReadyAccountHashRepositoryImpl {
            first_turn_decision_ready_account_hash_status: FirstTurnDecisionReadyAccountHash::new(),
        }
    }

    pub fn get_battle_ready_account_hash(&self) -> &FirstTurnDecisionReadyAccountHash {
        &self.first_turn_decision_ready_account_hash_status
    }

    pub fn get_instance() -> Arc<AsyncMutex<FirstTurnDecisionReadyAccountHashRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<FirstTurnDecisionReadyAccountHashRepositoryImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        FirstTurnDecisionReadyAccountHashRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl FirstTurnDecisionReadyAccountHashRepository for FirstTurnDecisionReadyAccountHashRepositoryImpl {
    async fn save_first_turn_decision_ready_account_list_hash(&mut self, accounts_vector: Vec<i32>, state: FirstTurnDecisionReadyAccountHashStatus) {
        println!("FirstTurnDecisionReadyAccountHashRepositoryImpl: save_first_turn_decision_account_list_hash()");

        for account_id in accounts_vector {
            self.first_turn_decision_ready_account_hash_status.set_user_ready_state(account_id, state);
        }
    }

    async fn save_first_turn_decision_ready_account_hash(&mut self, account_id: i32, state: FirstTurnDecisionReadyAccountHashStatus) {
        println!("FirstTurnDecisionReadyAccountHashRepositoryImpl: save_first_turn_decision_account_hash()");

        self.first_turn_decision_ready_account_hash_status.set_user_ready_state(account_id, state);
    }

    async fn get_first_turn_decision_ready_account_hash_status(&mut self, account_unique_id: i32) -> FirstTurnDecisionReadyAccountHashStatus {
        println!("FirstTurnDecisionReadyAccountHashRepositoryImpl: get_account_status()");

        let first_turn_decision_ready_status = self.first_turn_decision_ready_account_hash_status.get_user_ready_state(account_unique_id);
        return *first_turn_decision_ready_status.unwrap()
    }
}
