use bcrypt::BcryptError;
use crate::account::entity::account::Account;

#[derive(Debug)]
pub struct AccountModifyRequest {
    user_id: String,
    password: String,
    new_password: String,
}

impl AccountModifyRequest {
    pub fn new(user_id: &str, password: String, new_password: String) -> Self {
        AccountModifyRequest {
            user_id: user_id.to_string(),
            password: password.to_string(),
            new_password: new_password.to_string(),
        }
    }

    pub fn to_account(&self) -> Result<Account, BcryptError> {
        Account::new(&self.user_id, &self.password)
    }

    pub fn password(&self) -> &str { &self.password }

    pub fn new_password(&self) -> &str { &self.new_password }
}