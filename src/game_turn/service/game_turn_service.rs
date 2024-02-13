use async_trait::async_trait;
use crate::game_turn::service::request::first_turn_decision_request::FirstTurnDecisionRequest;
use crate::game_turn::service::response::first_turn_decision_response::FirstTurnDecisionResponse;


#[async_trait]
pub trait GameTurnService {

    async fn first_turn_decision_object(&mut self, decide_first_turn_request: FirstTurnDecisionRequest) -> FirstTurnDecisionResponse;
}