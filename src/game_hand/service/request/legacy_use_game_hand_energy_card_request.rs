#[derive(Debug)]
pub struct LegacyUseGameHandEnergyCardRequest {
    session_id: String,
    unit_number: String,
    energy_card_id: String,
}

impl LegacyUseGameHandEnergyCardRequest {
    pub fn new(session_id: String, unit_number: String, energy_card_id: String) -> Self {
        LegacyUseGameHandEnergyCardRequest {
            session_id: session_id.to_string(),
            unit_number: unit_number.to_string(),
            energy_card_id: energy_card_id.to_string(),
        }
    }

    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }

    pub fn get_unit_number(&self) -> &str {
        &self.unit_number
    }

    pub fn get_energy_card_id(&self) -> &str {
        &self.energy_card_id
    }
}