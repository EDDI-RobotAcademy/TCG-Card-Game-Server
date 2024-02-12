use async_trait::async_trait;

use crate::first_turn_decision_ready_account_hash::service::request::first_turn_decision_ready_account_hash_request::FirstTurnDecisionReadyAccountHashRequest;
use crate::first_turn_decision_ready_account_hash::service::request::check_first_turn_decision_prepare_request::CheckFirstTurnDecisionPrepareRequest;
use crate::first_turn_decision_ready_account_hash::service::response::first_turn_decision_ready_account_hash_response::FirstTurnDecisionReadyAccountHashResponse;
use crate::first_turn_decision_ready_account_hash::service::response::check_first_turn_decision_prepare_response::CheckFirstTurnDecisionPrepareResponse;

#[async_trait]
pub trait FirstTurnDecisionReadyAccountHashService {
    // async fn start_monitor(&mut self);
    async fn check_ready_for_first_turn_decision(&self, first_turn_decision_ready_account_hash_request: FirstTurnDecisionReadyAccountHashRequest) -> FirstTurnDecisionReadyAccountHashResponse;
    async fn check_prepare_for_first_turn_decision(&self, check_first_turn_decision_prepare_request: CheckFirstTurnDecisionPrepareRequest) -> CheckFirstTurnDecisionPrepareResponse;
}