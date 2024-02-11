use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirstTurnDecisionWaitQueueResponse {
    session_id1: i32,
    choice1: String,
    session_id2: i32,
    choice2: String,
}

impl FirstTurnDecisionWaitQueueResponse {
    pub fn new(session_id1: i32,choice1: String,
               session_id2: i32,choice2: String,) -> Self {
        FirstTurnDecisionWaitQueueResponse {
            session_id1,
            choice1,
            session_id2,
            choice2,
        }
    }
    pub fn get_session_id1(&self)-> i32 {self.session_id1}
    pub fn get_choice1(&self)-> &str {&self.choice1}
    pub fn get_session_id2(&self)-> i32 {self.session_id2}
    pub fn get_choice2(&self)-> &str {&self.choice2}



}
