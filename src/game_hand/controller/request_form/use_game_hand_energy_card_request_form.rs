use crate::game_hand::service::request::use_game_hand_energy_card_request::UseGameHandEnergyCardRequest;

#[derive(Debug)]
pub struct UseGameHandEnergyCardRequestForm {
    session_id: String,
    unit_number: String,
    energy_card_id: String,
}

impl UseGameHandEnergyCardRequestForm {
    pub fn new(session_id: String, unit_number: String, energy_card_id: String) -> Self {
        UseGameHandEnergyCardRequestForm {
            session_id,
            unit_number,
            energy_card_id,
        }
    }
    pub fn to_use_game_hand_energy_card_request(&self) -> UseGameHandEnergyCardRequest {
        UseGameHandEnergyCardRequest::new(
            self.session_id.clone(),
            self.unit_number.clone(),
            self.energy_card_id.clone()
        )
    }
}