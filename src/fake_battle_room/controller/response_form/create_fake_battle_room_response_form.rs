use serde::{Deserialize, Serialize};
use crate::account::service::response::account_login_response::AccountLoginResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFakeBattleRoomResponseForm {
    first_fake_session: String,
    second_fake_session: String
}

impl CreateFakeBattleRoomResponseForm {
    pub fn new(first_fake_session: String,
               second_fake_session: String) -> Self {

        Self { first_fake_session, second_fake_session }
    }

    pub fn from_login_response(
        first_login_response: AccountLoginResponse,
        second_login_response: AccountLoginResponse) -> CreateFakeBattleRoomResponseForm {

        CreateFakeBattleRoomResponseForm::new(
            first_login_response.get_redis_token().to_string(),
            second_login_response.get_redis_token().to_string())
    }
}