use async_trait::async_trait;
use crate::game_round::service::request::game_turn_request::GameRoundRequest;
use crate::game_round::service::request::next_game_turn_request::NextGameRoundRequest;
use crate::game_round::service::response::game_turn_response::GameRoundResponse;
use crate::game_round::service::response::next_game_turn_response::NextGameRoundResponse;

#[async_trait]
pub trait GameRoundService {
    async fn create_game_round_object(&mut self, game_round_request: GameRoundRequest) -> GameRoundResponse;
    async fn next_game_round_object(&mut self, game_round_request: NextGameRoundRequest) -> NextGameRoundResponse;
}
