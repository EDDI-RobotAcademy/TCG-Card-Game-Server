#[derive(Debug)]
pub struct UseGameHandUnitCardRequest {
    session_id: String,
    unit_number: String,
}

impl UseGameHandUnitCardRequest {
    pub fn new(session_id: String, unit_number: String) -> Self {
        UseGameHandUnitCardRequest {
            session_id: session_id.to_string(),
            unit_number: unit_number.to_string(),
        }
    }

    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }

    pub fn get_unit_number(&self) -> &str {
        &self.unit_number
    }
}