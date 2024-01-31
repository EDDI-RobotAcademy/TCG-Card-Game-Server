#[derive(Debug)]
pub struct GameTurnRequest {
    session_id: String,
}

impl GameTurnRequest {
    pub fn new(session_id: String) -> Self {
        GameTurnRequest {
            session_id: session_id.to_string(),
        }
    }

    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }
}
