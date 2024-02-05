#[derive(Debug)]
pub struct NextGameRoundRequest {
    session_id: String,
}

impl NextGameRoundRequest {
    pub fn new(session_id: String) -> Self {
        NextGameRoundRequest {
            session_id: session_id.to_string(),
        }
    }

    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }
}
