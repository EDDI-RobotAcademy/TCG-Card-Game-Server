#[derive(Debug)]
pub struct CheckFirstTurnDecisionPrepareRequest {
    session_id: String,
}

impl CheckFirstTurnDecisionPrepareRequest {
    pub fn new(session_id: String) -> Self {
        CheckFirstTurnDecisionPrepareRequest {
            session_id: session_id.to_string(),
        }
    }

    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }
}
