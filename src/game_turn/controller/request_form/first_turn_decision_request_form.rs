

#[derive(Debug)]
pub struct FirstTurnDecisionRequestForm {
    session_id: String,

}

impl FirstTurnDecisionRequestForm {
    pub fn new(session_id: String,) -> Self {
        FirstTurnDecisionRequestForm {
            session_id,

        }
    }
    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }

}