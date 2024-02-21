use async_trait::async_trait;
use crate::rockpaperscissors::service::request::check_draw_choice_request::CheckDrawChoiceRequest;
use crate::rockpaperscissors::service::request::check_opponent_hashmap_request::CheckOpponentHashmapRequest;
use crate::rockpaperscissors::service::request::check_rockpaperscissors_winner_request:: CheckRockpaperscissorsWinnerRequest;
use crate::rockpaperscissors::service::request::wait_hashmap_request::WaitHashmapRequest;
use crate::rockpaperscissors::service::response::check_opponent_hashmap_response::CheckOpponentHashmapResponse;
use crate::rockpaperscissors::service::response::check_rockpaperscissors_winner_response::CheckRockpaperscissorsWinnerResponse;
use crate::rockpaperscissors::service::response::wait_hashmap_response::WaitHashmapResponse;


#[async_trait]
pub trait RockpaperscissorsService {
    async fn insert_player_data_to_hashmap(&self, wait_queue_request: WaitHashmapRequest) -> WaitHashmapResponse;
    async fn check_draw_choice(&self, check_draw_choice_request: CheckDrawChoiceRequest);
    async fn check_rockpaperscissors_winner(&self, check_winner_request: CheckRockpaperscissorsWinnerRequest) -> CheckRockpaperscissorsWinnerResponse;
    async fn check_opponent_hashmap(&self, check_opponent_hashmap: CheckOpponentHashmapRequest) -> CheckOpponentHashmapResponse;
}