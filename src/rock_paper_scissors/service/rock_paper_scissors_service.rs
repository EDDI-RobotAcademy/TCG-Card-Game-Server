use async_trait::async_trait;

use crate::rock_paper_scissors::service::request::check_opponent_choice_request::CheckOpponentChoiceRequest;
use crate::rock_paper_scissors::service::request::check_rock_paper_scissors_winner_request::CheckRockPaperScissorsWinnerRequest;
use crate::rock_paper_scissors::service::request::register_rock_paper_scissors_wait_hash_request::RegisterRockPaperScissorsWaitHashRequest;

use crate::rock_paper_scissors::service::response::check_opponent_choice_response::CheckOpponentHashmapResponse;
use crate::rock_paper_scissors::service::response::check_rock_paper_scissors_winner_response::CheckRockPaperScissorsWinnerResponse;
use crate::rock_paper_scissors::service::response::register_rock_paper_scissors_wait_hash_response::RegisterRockPaperScissorsWaitHashResponse;


#[async_trait]
pub trait RockPaperScissorsService {
    async fn register_rock_paper_scissors_wait_hash(&self, register_rock_paper_scissors_wait_hash_request: RegisterRockPaperScissorsWaitHashRequest) -> RegisterRockPaperScissorsWaitHashResponse;
    async fn check_rock_paper_scissors_winner(&self, check_rock_paper_scissors_winner_request: CheckRockPaperScissorsWinnerRequest) -> CheckRockPaperScissorsWinnerResponse;
}