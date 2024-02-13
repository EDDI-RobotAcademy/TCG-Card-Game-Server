#[derive(Debug, PartialEq,Clone)]
pub struct FirstTurnDecisionRequest {
    session_id:String,

}

impl FirstTurnDecisionRequest {
    pub fn new(session_id:String) -> Self {
        FirstTurnDecisionRequest {
            session_id
        }
    }

    pub fn get_session_id(&self) -> &str {&self.session_id}

}