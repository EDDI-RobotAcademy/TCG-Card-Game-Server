#[derive(Debug)]
pub struct BattleMatchCancelRequest {
    session_id: String,
}

impl BattleMatchCancelRequest {
    pub fn new(session_id: String) -> Self {
        BattleMatchCancelRequest {
            session_id,
        }
    }

    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }
}