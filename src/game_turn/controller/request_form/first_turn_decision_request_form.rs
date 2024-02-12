



#[derive(Debug)]
pub struct FirstTurnDecisionRequestForm {
    pub(crate) session_id: String,
    pub(crate) choice: String,
}

impl FirstTurnDecisionRequestForm {
    pub fn new(session_id: String, choice: String) -> Self {
        FirstTurnDecisionRequestForm {
            session_id,
            choice
        }
    }

}