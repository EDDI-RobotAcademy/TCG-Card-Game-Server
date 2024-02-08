#[derive(Debug)]
pub struct GameDeckCardListRequest {
    session_id: String
}

impl GameDeckCardListRequest {
    pub fn new(session_id: String) -> Self { GameDeckCardListRequest { session_id } }
    pub fn get_session_id(&self) -> &str { &self.session_id }
}