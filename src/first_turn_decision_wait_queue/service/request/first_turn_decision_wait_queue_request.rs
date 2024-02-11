#[derive(Debug)]
pub struct FirstTurnDecisionWaitQueueRequest {
    pub(crate) session_id: String,
    pub(crate) choice: String,

}

impl FirstTurnDecisionWaitQueueRequest {
    pub fn new(session_id: String,choice:String) -> Self {
        FirstTurnDecisionWaitQueueRequest {
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