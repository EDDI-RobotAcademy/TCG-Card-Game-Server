use std::println;
use async_trait::async_trait;
use crate::game_turn::service::request::first_turn_decision_request::FirstTurnDecisionRequest;
use crate::game_turn::service::request::next_turn_request::NextTurnRequest;
use crate::game_turn::service::response::first_turn_decision_response::FirstTurnDecisionResponse;
use crate::game_turn::service::response::next_turn_response::NextTurnResponse;


#[async_trait]
pub trait GameTurnService {

    async fn next_turn(&mut self, next_turn_request: NextTurnRequest) -> NextTurnResponse;
}