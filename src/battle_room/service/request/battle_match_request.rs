#[derive(Debug)]
pub struct BattleMatchRequest {
    session_id: String,
}

impl BattleMatchRequest {
    pub fn new(session_id: String) -> Self {
        BattleMatchRequest {
            session_id: session_id.to_string(),
        }
    }

    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }
}