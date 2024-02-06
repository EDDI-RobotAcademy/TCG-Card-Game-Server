#[derive(Debug)]
pub struct EnergyBoostSupportRequestForm {
    session_id: String,
    unit_number: String,
    support_card_number: String,
}

impl EnergyBoostSupportRequestForm {
    pub fn new(session_id: String, unit_number: String, support_card_number: String) -> Self {
        EnergyBoostSupportRequestForm {
            session_id: session_id.to_string(),
            unit_number: unit_number.to_string(),
            support_card_number: support_card_number.to_string(),
        }
    }
    pub fn to_use_game_hand_unit_card_request(&self) -> EnergyBoostSupportRequest {
        EnergyBoostSupportRequest::new(
            self.session_id.clone(), self.unit_number.clone(), self.support_card_number.clone())
    }
}