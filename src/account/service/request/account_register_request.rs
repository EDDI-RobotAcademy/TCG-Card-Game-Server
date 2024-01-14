use bcrypt::BcryptError;
use crate::account::entity::account::Account;

#[derive(Debug)]
pub struct AccountRegisterRequest {
    user_id: String,
    password: String,
}

impl AccountRegisterRequest {
    pub fn new(user_id: &str, password: String) -> Self {
        AccountRegisterRequest {
            user_id: user_id.to_string(),
            password: password.to_string(),
        }
    }

    pub fn to_account(&self) -> Result<Account, BcryptError> {
        Account::new(&self.user_id, &self.password)
    }
}