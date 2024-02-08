#[derive(Debug)]
pub struct GameDeckCardShuffleRequest {
    session_id: String,
}

impl GameDeckCardShuffleRequest {
    pub fn new(session_id: String) -> Self {
        GameDeckCardShuffleRequest {
            session_id,
        }
    }

    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }
}
