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
}