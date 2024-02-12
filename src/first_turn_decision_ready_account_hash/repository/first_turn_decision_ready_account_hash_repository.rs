use std::error::Error;
use async_trait::async_trait;
use crate::first_turn_decision_ready_account_hash::entity::first_turn_decision_ready_account_hash_status::FirstTurnDecisionReadyAccountHashStatus;

#[async_trait]
pub trait FirstTurnDecisionReadyAccountHashRepository {
    async fn save_first_turn_decision_ready_account_list_hash(&mut self, accounts_vector: Vec<i32>, state: FirstTurnDecisionReadyAccountHashStatus);
    async fn save_first_turn_decision_ready_account_hash(&mut self, account_id: i32, state: FirstTurnDecisionReadyAccountHashStatus);
    async fn get_first_turn_decision_ready_account_hash_status(&mut self, account_unique_id: i32) -> FirstTurnDecisionReadyAccountHashStatus;
}