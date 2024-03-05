use async_trait::async_trait;

use crate::mulligan::controller::request_form::mulligan_request_form::MulliganRequestForm;
use crate::mulligan::controller::response_form::mulligan_response_form::MulliganResponseForm;

#[async_trait]
pub trait MulliganController {
    async fn execute_mulligan_procedure(
        &self, change_first_hand_request_form: MulliganRequestForm)
        -> MulliganResponseForm;
}
