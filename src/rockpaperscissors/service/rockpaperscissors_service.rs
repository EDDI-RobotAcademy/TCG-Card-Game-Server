use async_trait::async_trait;
use crate::rockpaperscissors::service::request::check_winner_request::CheckWinnerRequest;
use crate::rockpaperscissors::service::request::wait_queue_request::WaitQueueRequest;
use crate::rockpaperscissors::service::response::check_winner_response::CheckWinnerResponse;
use crate::rockpaperscissors::service::response::wait_queue_response::WaitQueueResponse;


#[async_trait]
pub trait RockpaperscissorsService {
    async fn enqueue_player_tuple_to_wait_queue(&self, wait_queue_request: WaitQueueRequest) -> WaitQueueResponse;
    async fn check_winner(&self, check_winner_request: CheckWinnerRequest) -> CheckWinnerResponse;
}