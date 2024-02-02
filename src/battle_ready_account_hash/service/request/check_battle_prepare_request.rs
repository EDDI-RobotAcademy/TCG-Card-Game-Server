#[derive(Debug)]
pub struct CheckBattlePrepareRequest {
    session_id: String,
}

impl CheckBattlePrepareRequest {
    pub fn new(session_id: String) -> Self {
        CheckBattlePrepareRequest {
            session_id: session_id.to_string(),
        }
    }

    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }
}
