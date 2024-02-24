use crate::account::service::request::account_login_request::AccountLoginRequest;
use crate::fake_battle_room::service::request::create_battle_fake_room_request::CreateFakeBattleRoomRequest;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;

#[derive(Debug)]
pub struct CreateFakeBattleRoomRequestForm {
    first_fake_test_account_id: String,
    first_fake_test_account_password: String,
    second_fake_test_account_id: String,
    second_fake_test_account_password: String
}

impl CreateFakeBattleRoomRequestForm {
    pub fn new(
        first_fake_test_account_id: String,
        first_fake_test_account_password: String,
        second_fake_test_account_id: String,
        second_fake_test_account_password: String) -> Self {

        CreateFakeBattleRoomRequestForm {
            first_fake_test_account_id: first_fake_test_account_id.to_string(),
            first_fake_test_account_password: first_fake_test_account_password.to_string(),
            second_fake_test_account_id: second_fake_test_account_id.to_string(),
            second_fake_test_account_password: second_fake_test_account_password.to_string()
        }
    }

    pub fn get_first_fake_test_account_id(&self) -> &str {
        &self.first_fake_test_account_id
    }

    pub fn get_first_fake_test_account_password(&self) -> &str {
        &self.first_fake_test_account_password
    }

    pub fn get_second_fake_test_account_id(&self) -> &str {
        &self.second_fake_test_account_id
    }

    pub fn get_second_fake_test_account_password(&self) -> &str {
        &self.second_fake_test_account_password
    }

    pub fn to_first_fake_test_login_request(&self) -> AccountLoginRequest {
        AccountLoginRequest::new(
            &self.first_fake_test_account_id,
            self.first_fake_test_account_password.clone(),
        )
    }

    pub fn to_second_fake_test_login_request(&self) -> AccountLoginRequest {
        AccountLoginRequest::new(
            &self.first_fake_test_account_id,
            self.first_fake_test_account_password.clone(),
        )
    }

    pub fn to_create_fake_battle_room_request(&self) -> CreateFakeBattleRoomRequest {
        CreateFakeBattleRoomRequest::new(
            (&self.first_fake_test_account_id).parse().unwrap(),
            (&self.second_fake_test_account_id).parse().unwrap()
        )
    }
}
