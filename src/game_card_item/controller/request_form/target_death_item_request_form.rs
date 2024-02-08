use crate::game_card_support::controller::request_form::energy_boost_support_request_form::EnergyBoostSupportRequestForm;

#[derive(Debug)]
pub struct TargetDeathItemRequestForm {
    session_id: String,
    opponent_target_unit_index: String,
    item_card_id: String,
}

impl TargetDeathItemRequestForm {
    pub fn new(session_id: String, opponent_target_unit_index: String, item_card_id: String) -> Self {
        TargetDeathItemRequestForm {
            session_id: session_id.to_string(),
            opponent_target_unit_index: opponent_target_unit_index.to_string(),
            item_card_id: item_card_id.to_string(),
        }
    }
}