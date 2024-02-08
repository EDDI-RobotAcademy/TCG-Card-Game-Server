#[derive(Debug)]
pub struct GameDeckStartCardListRequest {
    deck_id: String,
    session_id: String,
}

impl GameDeckStartCardListRequest {
    pub fn new(deck_id: String, session_id: String) -> Self {
        GameDeckStartCardListRequest {
            deck_id,
            session_id,
        }
    }

    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }

    pub fn get_deck_id(&self) -> i32 {
        self.deck_id.parse().unwrap_or_default()
    }
}
