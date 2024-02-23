#[derive(Debug)]
pub struct RemainDeckCardCountRequest {
    session_id: String,
    who: String,
}

impl RemainDeckCardCountRequest {
    pub fn new(session_id: String,who:String) -> Self {
        RemainDeckCardCountRequest {
            session_id,
            who,
        }
    }

    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }
    pub fn get_who(&self) -> &str {
        &self.who
    }
}