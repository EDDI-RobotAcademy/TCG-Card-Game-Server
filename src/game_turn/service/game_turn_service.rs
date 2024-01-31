use async_trait::async_trait;
use crate::game_turn::service::request::game_turn_request::GameTurnRequest;
use crate::game_turn::service::request::next_game_turn_request::NextGameTurnRequest;
use crate::game_turn::service::response::game_turn_response::GameTurnResponse;
use crate::game_turn::service::response::next_game_turn_response::NextGameTurnResponse;

#[async_trait]
pub trait GameTurnService {
    async fn create_game_turn_object(&mut self, game_turn_request: GameTurnRequest) -> GameTurnResponse;
    async fn next_game_turn_object(&mut self, game_turn_request: NextGameTurnRequest) -> NextGameTurnResponse;
}