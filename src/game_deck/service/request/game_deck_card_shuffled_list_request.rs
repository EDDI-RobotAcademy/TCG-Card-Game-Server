#[derive(Debug)]
pub struct GameDeckCardShuffledListRequest {
    session_id: String,
}

impl GameDeckCardShuffledListRequest {
    pub fn new(session_id: String) -> Self {
        GameDeckCardShuffledListRequest {
            session_id,
        }
    }

    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }
}
