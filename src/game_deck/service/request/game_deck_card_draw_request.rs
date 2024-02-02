#[derive(Debug)]
pub struct GameDeckCardDrawRequest {
    session_id: String,
    draw_count: i32,
}

impl GameDeckCardDrawRequest {
    pub fn new(session_id: String, draw_count: i32) -> Self {
        GameDeckCardDrawRequest {
            session_id,
            draw_count,
        }
    }

    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }

    pub fn get_draw_count(&self) -> i32 {
        self.draw_count
    }
}
