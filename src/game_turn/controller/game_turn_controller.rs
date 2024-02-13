use async_trait::async_trait;

use crate::game_turn::controller::request_form::first_turn_decision_wait_queue_request_form::FirstTurnDecisionWaitQueueRequestForm;
use crate::game_turn::controller::response_form::first_turn_decision_wait_queue_response_form::FirstTurnDecisionWaitQueueResponseForm;
use crate::game_turn::controller::request_form::first_turn_decision_request_form::FirstTurnDecisionRequestForm;
use crate::game_turn::controller::response_form::first_turn_decision_response_form::FirstTurnDecisionResponseForm;

#[async_trait]
pub trait  GameTurnController {
    async fn execute_first_turn_decision_wait_queue_procedure(
        &self, first_turn_decision_wait_queue_request_form: FirstTurnDecisionWaitQueueRequestForm) -> FirstTurnDecisionWaitQueueResponseForm;
    async fn execute_first_turn_decision_procedure(
        &self, first_turn_decision_request_form: FirstTurnDecisionRequestForm) -> FirstTurnDecisionResponseForm;
}