#[derive(Debug)]
pub struct AccountSessionLogoutRequest {
    session_id: String,
}

impl AccountSessionLogoutRequest {
    pub fn new(session_id: String) -> Self {
        AccountSessionLogoutRequest {
            session_id: session_id.to_string(),
        }
    }

    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }
}