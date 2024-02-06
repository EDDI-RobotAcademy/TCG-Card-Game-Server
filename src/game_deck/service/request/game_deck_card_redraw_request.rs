#[derive(Debug)]
pub struct GameDeckCardRedrawRequest {
    session_id: String,
    redraw_card_count: i32,
}

impl GameDeckCardRedrawRequest {
    pub fn new(session_id: String, redraw_card_count: i32) -> Self {
        GameDeckCardRedrawRequest {
            session_id,
            redraw_card_count
        }
    }
    pub fn get_session_id(&self) -> &str { &self.session_id }
    pub fn get_redraw_card_count(&self) -> i32 { self.redraw_card_count }
}