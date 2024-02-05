#[derive(Debug)]
pub struct DecideFirstTurnRequest {
    session_id: String,
    gesture: String,
}

impl DecideFirstTurnRequest {
    pub fn new(session_id: String, gesture: String) -> Self {
        DecideFirstTurnRequest {
            session_id: session_id.to_string(),
            gesture: gesture.to_string()
        }
    }

    pub fn get_session_id(&self) -> &str { &self.session_id }

    pub fn get_gesture(&self) -> &str { &self.gesture }
}