use crate::battle_room::service::request::find_opponent_by_account_id_request::FindOpponentByAccountIdRequest;
use crate::mulligan::service::request::check_opponent_mulligan_timer_request::CheckOpponentMulliganTimerRequest;
use crate::mulligan::service::request::is_opponent_mulligan_finished_request::IsOpponentMulliganFinishedRequest;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;


#[derive(Debug)]
pub struct CheckOpponentMulliganStatusRequestForm {
    session_id: String,
}

impl CheckOpponentMulliganStatusRequestForm {
    pub fn new(
        session_id: String,
    ) -> Self {

        CheckOpponentMulliganStatusRequestForm {
            session_id,
        }
    }

    pub fn to_get_value_with_key_request(
        &self) -> GetValueWithKeyRequest {

        GetValueWithKeyRequest::new(&self.session_id)
    }

    pub fn to_find_opponent_by_account_id_request(
        &self,
        account_unique_id: i32) -> FindOpponentByAccountIdRequest {

        FindOpponentByAccountIdRequest::new(
            account_unique_id)
    }

    pub fn to_is_opponent_mulligan_finished_request(
        &self,
        opponent_unique_id: i32) -> IsOpponentMulliganFinishedRequest {

        IsOpponentMulliganFinishedRequest::new(
            opponent_unique_id)
    }

    pub fn to_check_opponent_mulligan_timer_request(
        &self,
        opponent_unique_id: i32) -> CheckOpponentMulliganTimerRequest {

        CheckOpponentMulliganTimerRequest::new(
            opponent_unique_id)
    }
}