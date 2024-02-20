use async_trait::async_trait;
use crate::rockpaperscissors::service::request::check_winner_request::CheckWinnerRequest;
use crate::rockpaperscissors::service::request::wait_hashmap_request::WaitHashmapRequest;
use crate::rockpaperscissors::service::response::check_winner_response::CheckWinnerResponse;
use crate::rockpaperscissors::service::response::wait_hashmap_response::WaitHashmapResponse;


#[async_trait]
pub trait RockpaperscissorsService {
    async fn insert_player_data_to_hashmap(&self, wait_queue_request: WaitHashmapRequest) -> WaitHashmapResponse;
    async fn check_rockpaperscissors_winner(&self, check_winner_request: CheckWinnerRequest) -> CheckWinnerResponse;
}