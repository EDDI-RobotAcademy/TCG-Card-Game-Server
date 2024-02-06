#[derive(Debug, PartialEq)]
pub struct DecideFirstTurnRequest {
    session_id: String,
    choice: Gesture,
}
#[derive(Debug, PartialEq,Clone)]
pub(crate) enum Gesture {
    Rock,
    Paper,
    Scissors,
}


impl DecideFirstTurnRequest {
    pub fn new(session_id: String, choice: Gesture) -> Self {
        DecideFirstTurnRequest {
            session_id: session_id.to_string(),
            choice
        }
    }

    pub fn get_session_id(&self) -> &str { &self.session_id }

    pub fn get_choice(&self) -> Gesture { self.choice.clone() }
}