use async_trait::async_trait;

use crate::mulligan::controller::request_form::check_opponent_mulligan_status_request_form::CheckOpponentMulliganStatusRequestForm;
use crate::mulligan::controller::request_form::mulligan_request_form::MulliganRequestForm;

use crate::mulligan::controller::response_form::check_opponent_mulligan_status_response_form::CheckOpponentMulliganStatusResponseForm;
use crate::mulligan::controller::response_form::mulligan_response_form::MulliganResponseForm;

#[async_trait]
pub trait MulliganController {
    async fn execute_mulligan_procedure(
        &self, change_first_hand_request_form: MulliganRequestForm)
        -> MulliganResponseForm;
    async fn check_opponent_mulligan_status(
        &self, check_opponent_mulligan_status_request_form: CheckOpponentMulliganStatusRequestForm)
        -> CheckOpponentMulliganStatusResponseForm;
}
