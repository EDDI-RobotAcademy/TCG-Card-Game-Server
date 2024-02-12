use async_trait::async_trait;

use crate::game_turn::controller::request_form::first_turn_decision_request_form::FirstTurnDecisionRequestForm;
use crate::game_turn::controller::response_form::first_turn_decision_response_form::FirstTurnDecisionResponseForm;


#[async_trait]
pub trait  GameTurnController {
    async fn execute_first_turn_decision_procedure(
        &self, decide_first_turn_request_form: FirstTurnDecisionRequestForm) -> FirstTurnDecisionResponseForm;
}