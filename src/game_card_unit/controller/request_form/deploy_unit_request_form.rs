use crate::game_card_support::controller::request_form::energy_boost_support_request_form::EnergyBoostSupportRequestForm;

pub struct DeployUnitRequestForm{
    session_id: String,
    unit_id: String,
    unit_place_index: String,
}

impl DeployUnitRequestForm {
    pub fn new(session_id: String, unit_id: String, unit_place_index: String) -> Self {
        DeployUnitRequestForm {
            session_id: session_id.to_string(),
            unit_id: unit_id.to_string(),
            unit_place_index: unit_place_index.to_string(),
        }
    }

    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    pub fn unit_id(&self) -> &str {
        &self.unit_id
    }

    pub fn unit_place_index(&self) -> &str {
        &self.unit_place_index
    }
}