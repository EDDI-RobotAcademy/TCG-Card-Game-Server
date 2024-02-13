


#[derive(Debug)]
pub struct FirstTurnDecisionWaitQueueRequestForm {
    session_id: String,
    choice: String,
}

impl FirstTurnDecisionWaitQueueRequestForm {
    pub fn new(session_id: String, choice: String) -> Self {
        FirstTurnDecisionWaitQueueRequestForm {
            session_id,
            choice
        }
    }
    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }
    pub fn get_choice(&self) -> &str {
        &self.choice
    }
}
