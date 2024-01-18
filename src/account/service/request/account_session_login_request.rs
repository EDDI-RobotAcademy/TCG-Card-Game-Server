#[derive(Debug)]
pub struct AccountSessionLoginRequest {
    session_id: String,
}

impl AccountSessionLoginRequest {
    pub fn new(session_id: String) -> Self {
        AccountSessionLoginRequest {
            session_id: session_id.to_string(),
        }
    }

    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }
}