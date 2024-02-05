use async_trait::async_trait;
use crate::game_round::service::request::game_turn_request::GameRoundRequest;
use crate::game_round::service::response::game_turn_response::GameRoundResponse;
use crate::game_turn::service::request::decide_first_turn_request::DecideFirstTurnRequest;
use crate::game_turn::service::response::decide_first_turn_response::DecideFirstTurnResponse;


#[async_trait]
pub trait GameTurnService {

    async fn decide_first_turn_object(&mut self, decide_first_turn_request: DecideFirstTurnRequest) -> DecideFirstTurnResponse;
}