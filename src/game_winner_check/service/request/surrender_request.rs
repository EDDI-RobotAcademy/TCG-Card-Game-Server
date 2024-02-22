#[derive(Debug)]
pub struct SurrenderRequest {
    session_id: String,
}

impl SurrenderRequest {
    pub fn new(session_id: String) -> Self {
        SurrenderRequest {
            session_id: session_id.to_string(),
        }
    }

    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }
}