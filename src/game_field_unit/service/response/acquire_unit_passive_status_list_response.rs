use crate::game_card_unit::entity::passive_status::PassiveStatus;

#[derive(Debug, Clone)]
pub struct AcquireUnitPassiveStatusListResponse {
    passive_status_effect_list: Vec<PassiveStatus>,
}

impl AcquireUnitPassiveStatusListResponse {
    pub fn new(passive_status_effect_list: Vec<PassiveStatus>) -> Self {
        Self { passive_status_effect_list }
    }

    pub fn get_passive_status_effect_list(&self) -> &Vec<PassiveStatus> {
        &self.passive_status_effect_list
    }
}