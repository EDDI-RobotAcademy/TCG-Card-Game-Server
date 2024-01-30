#[derive(Debug)]
pub struct BattleReadyAccountHashRequest {
    session_id: String,
}

impl BattleReadyAccountHashRequest {
    pub fn new(session_id: String) -> Self {
        BattleReadyAccountHashRequest {
            session_id: session_id.to_string(),
        }
    }

    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }
}
