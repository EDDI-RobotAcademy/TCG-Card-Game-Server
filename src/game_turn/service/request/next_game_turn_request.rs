#[derive(Debug)]
pub struct NextGameTurnRequest {
    session_id: String,
}

impl NextGameTurnRequest {
    pub fn new(session_id: String) -> Self {
        NextGameTurnRequest {
            session_id: session_id.to_string(),
        }
    }

    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }
}
