use bcrypt::BcryptError;
use crate::account::entity::account::Account;

#[derive(Debug)]
pub struct AccountDeleteRequest {
    user_id: String,
    password: String,
    session_id: String
}

impl AccountDeleteRequest {
    pub fn new(user_id: &str, password: String, session_id: String) -> Self {
        AccountDeleteRequest {
            user_id: user_id.to_string(),
            password: password.to_string(),
            session_id: session_id.to_string(),
        }
    }

    pub fn user_id(&self) -> &str { &self.user_id }

    pub fn password(&self) -> &str { &self.password }

    pub fn session_id(&self) -> &str { &self.session_id }
}
