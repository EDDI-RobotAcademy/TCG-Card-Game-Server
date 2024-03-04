use async_trait::async_trait;

use crate::rock_paper_scissors::controller::request_form::rock_paper_scissors_request_form::RockPaperScissorsRequestForm;
use crate::rock_paper_scissors::controller::response_form::rock_paper_scissors_response_form::RockPaperScissorsResponseForm;
use crate::rock_paper_scissors::controller::request_form::check_rock_paper_scissors_winner_request_form::CheckRockPaperScissorsWinnerRequestForm;
use crate::rock_paper_scissors::controller::response_form::check_rock_paper_scissors_winner_response_form::CheckRockPaperScissorsWinnerResponseForm;

#[async_trait]
pub trait  RockPaperScissorsController {
    async fn execute_rock_paper_scissors_procedure(
        &self, rock_paper_scissors_request_form: RockPaperScissorsRequestForm)
        -> RockPaperScissorsResponseForm;
    async fn execute_check_rock_paper_scissors_winner_procedure(
        &self, check_rock_paper_scissors_winner_request_form: CheckRockPaperScissorsWinnerRequestForm)
        -> CheckRockPaperScissorsWinnerResponseForm;
}