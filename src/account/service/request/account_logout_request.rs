#[derive(Debug)]
pub struct AccountLogoutRequest {
    session_id: String,
}

impl AccountLogoutRequest {
    pub fn new(session_id: String) -> Self {
        AccountLogoutRequest {
            session_id: session_id.to_string(),
        }
    }

    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }
}