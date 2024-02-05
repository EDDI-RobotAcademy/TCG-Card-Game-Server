#[derive(Debug)]
pub struct GameRoundRequest {
    session_id: String,
}

impl GameRoundRequest {
    pub fn new(session_id: String) -> Self {
        GameRoundRequest {
            session_id: session_id.to_string(),
        }
    }

    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }
}
