use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;

#[derive(Debug)]
pub struct TurnEndRequestForm {
    session_id: String,

}

impl TurnEndRequestForm {
    pub fn new(session_id: String,) -> Self {
        TurnEndRequestForm {
            session_id,

        }
    }

    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }

    pub fn to_session_validation_request(&self) -> GetValueWithKeyRequest {
        GetValueWithKeyRequest::new(self.session_id.clone().as_str())
    }
}