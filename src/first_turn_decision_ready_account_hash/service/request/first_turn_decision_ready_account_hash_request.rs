#[derive(Debug)]
pub struct FirstTurnDecisionReadyAccountHashRequest {
    session_id: String,
}

impl FirstTurnDecisionReadyAccountHashRequest {
    pub fn new(session_id: String) -> Self {
        FirstTurnDecisionReadyAccountHashRequest {
            session_id: session_id.to_string(),
        }
    }

    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }
}
