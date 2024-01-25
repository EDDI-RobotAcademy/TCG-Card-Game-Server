#[derive(Debug)]
pub struct BattleReadyRequest {
    session_id: String,
}

impl BattleReadyRequest {
    pub fn new(session_id: String) -> Self {
        BattleReadyRequest {
            session_id: session_id.to_string(),
        }
    }

    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }
}
