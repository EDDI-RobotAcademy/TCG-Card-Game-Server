use crate::battle_room::service::request::find_opponent_by_account_id_request::FindOpponentByAccountIdRequest;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;
use crate::rock_paper_scissors::service::request::check_draw_choice_request::CheckDrawChoiceRequest;
use crate::rock_paper_scissors::service::request::register_rock_paper_scissors_wait_hash_request::RegisterRockPaperScissorsWaitHashRequest;

#[derive(Debug)]
pub struct RockPaperScissorsRequestForm {
    session_id: String,
    choice: String,
}

impl RockPaperScissorsRequestForm {
    pub fn new(session_id: String, choice: String) -> Self {
        RockPaperScissorsRequestForm {
            session_id,
            choice
        }
    }

    pub fn get_choice(&self) -> &str { &self.choice }

    pub fn to_session_validation_request(&self) -> GetValueWithKeyRequest {
        GetValueWithKeyRequest::new(self.session_id.clone().as_str())
    }

    pub fn to_find_opponent_by_account_id_request(
        &self,
        account_unique_id: i32) -> FindOpponentByAccountIdRequest {

        FindOpponentByAccountIdRequest::new(
            account_unique_id)
    }

    pub fn to_register_rock_paper_scissors_wait_hash_request(
        &self,
        account_unique_id: i32,
        opponent_unique_id: i32,
        choice: &str
    ) -> RegisterRockPaperScissorsWaitHashRequest {

        RegisterRockPaperScissorsWaitHashRequest::new(
            account_unique_id,
            opponent_unique_id,
            choice.to_string())
    }
}