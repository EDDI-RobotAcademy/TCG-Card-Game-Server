use crate::battle_room::service::request::find_opponent_by_account_id_request::FindOpponentByAccountIdRequest;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;
use crate::rock_paper_scissors::service::request::check_rock_paper_scissors_winner_request::CheckRockPaperScissorsWinnerRequest;

#[derive(Debug)]
pub struct CheckRockPaperScissorsWinnerRequestForm {
    session_id: String,
}

impl CheckRockPaperScissorsWinnerRequestForm {
    pub fn new(session_id: String ) -> Self {
        CheckRockPaperScissorsWinnerRequestForm {
            session_id,

        }
    }

    pub fn to_session_validation_request(
        &self) -> GetValueWithKeyRequest {

        GetValueWithKeyRequest::new(
            self.session_id.clone().as_str())
    }

    pub fn to_find_opponent_by_account_id_request(
        &self,
        account_unique_id: i32) -> FindOpponentByAccountIdRequest {

        FindOpponentByAccountIdRequest::new(
            account_unique_id)
    }

    pub fn to_check_rock_paper_scissors_winner_request(
        &self,
        account_unique_id: i32,
        opponent_unique_id: i32
    ) -> CheckRockPaperScissorsWinnerRequest {

        CheckRockPaperScissorsWinnerRequest::new(
            account_unique_id, opponent_unique_id)
    }
}