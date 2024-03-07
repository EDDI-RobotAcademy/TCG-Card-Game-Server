use async_trait::async_trait;
use crate::game_winner_check::service::request::check_main_character_request::CheckMainCharacterRequest;
use crate::game_winner_check::service::request::surrender_request::SurrenderRequest;
use crate::game_winner_check::service::response::surrender_response::SurrenderResponse;

#[async_trait]
pub trait GameWinnerCheckService {
    async fn set_game_winner(&mut self, check_main_character_request: CheckMainCharacterRequest);
    async fn set_game_winner_by_surrender(&mut self, surrender_request: SurrenderRequest) -> SurrenderResponse;
}