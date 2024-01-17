use bcrypt::BcryptError;
use crate::account::entity::account::Account;

#[derive(Debug)]
pub struct AccountLoginRequest {
    user_id: String,
    pub(crate) password: String,
}

impl AccountLoginRequest {
    pub fn new(user_id: &str, password: String) -> Self {
        AccountLoginRequest {
            user_id: user_id.to_string(),
            password: password.to_string(),
        }
    }

    pub fn to_account(&self) -> Result<Account, BcryptError> {
        Account::new(&self.user_id, &self.password)
    }
}