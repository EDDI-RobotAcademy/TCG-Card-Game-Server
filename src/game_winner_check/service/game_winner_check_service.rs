use async_trait::async_trait;
use crate::game_winner_check::service::request::check_main_character_request::CheckMainCharacterRequest;

#[async_trait]
pub trait GameWinnerCheckService {
    async fn check_health_of_main_character_for_setting_game_winner(&mut self, check_main_character_request: CheckMainCharacterRequest);
}