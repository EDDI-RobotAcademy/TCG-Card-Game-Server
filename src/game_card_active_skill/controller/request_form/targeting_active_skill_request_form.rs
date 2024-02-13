use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;

pub struct TargetingActiveSkillRequestForm {
    session_id: String,
    unit_card_index: String,
    opponent_target_card_index: String,
}

impl TargetingActiveSkillRequestForm {
    pub fn new(session_id: String, unit_card_index: String, opponent_target_card_index: String) -> Self {
        TargetingActiveSkillRequestForm {
            session_id: session_id.to_string(),
            unit_card_index: unit_card_index.to_string(),
            opponent_target_card_index: opponent_target_card_index.to_string(),
        }
    }

    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }

    pub fn get_unit_card_index(&self) -> &str {
        &self.unit_card_index
    }

    pub fn get_opponent_target_card_index(&self) -> &str {
        &self.opponent_target_card_index
    }

    pub fn to_session_validation_request(&self) -> GetValueWithKeyRequest {
        GetValueWithKeyRequest::new(self.session_id.clone().as_str())
    }
}
