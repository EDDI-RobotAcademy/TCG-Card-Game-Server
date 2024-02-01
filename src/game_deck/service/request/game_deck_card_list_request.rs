#[derive(Debug)]
pub struct GameDeckCardListRequest {
    deck_id: i32,
    session_id: String,
}

impl GameDeckCardListRequest {
    pub fn new(deck_id: i32, session_id: String) -> Self {
        GameDeckCardListRequest {
            deck_id,
            session_id,
        }
    }

    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }

    pub fn get_deck_id(&self) -> i32 {
        self.deck_id
    }
}
