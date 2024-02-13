use async_trait::async_trait;

use crate::game_turn::controller::request_form::first_turn_decision_wait_queue_request_form::FirstTurnDecisionWaitQueueRequestForm;
use crate::game_turn::controller::response_form::first_turn_decision_wait_queue_response_form::FirstTurnDecisionWaitQueueResponseForm;
use crate::game_turn::controller::request_form::first_turn_decision_request_form::FirstTurnDecisionRequestForm;
use crate::game_turn::controller::request_form::turn_end_request_form::TurnEndRequestForm;
use crate::game_turn::controller::response_form::first_turn_decision_response_form::FirstTurnDecisionResponseForm;
use crate::game_turn::controller::response_form::turn_end_response_form::TurnEndResponseForm;

#[async_trait]
pub trait  GameTurnController {
    // TODO: Need Refactor
    async fn execute_first_turn_decision_wait_queue_procedure(
        &self, first_turn_decision_wait_queue_request_form: FirstTurnDecisionWaitQueueRequestForm) -> FirstTurnDecisionWaitQueueResponseForm;
    async fn execute_first_turn_decision_procedure(
        &self, first_turn_decision_request_form: FirstTurnDecisionRequestForm) -> FirstTurnDecisionResponseForm;
    async fn request_turn_end(
        &self, turn_end_request_form: TurnEndRequestForm) -> TurnEndResponseForm;
}