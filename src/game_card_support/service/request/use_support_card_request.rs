use crate::game_hand::service::request::use_game_hand_unit_card_request::UseGameHandUnitCardRequest;

#[derive(Clone)]
pub struct UseSupportCardRequest {
    session_id: String,
    unit_number: String,
    support_card_number: String,
}

impl UseSupportCardRequest {
    pub fn new(session_id: String, unit_number: String, support_card_number: String) -> Self {
        UseSupportCardRequest {
            session_id: session_id.to_string(),
            unit_number: unit_number.to_string(),
            support_card_number: support_card_number.to_string(),
        }
    }

    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }

    pub fn get_unit_number(&self) -> &str {
        &self.unit_number
    }


    pub fn get_support_card_number(&self) -> &str {
        &self.support_card_number
    }
}
