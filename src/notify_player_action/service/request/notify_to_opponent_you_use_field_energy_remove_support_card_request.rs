#[derive(Debug)]
pub struct NotifyToOpponentYouUseFieldEnergyRemoveSupportCardRequest {
    opponent_unique_id: i32,
    usage_hand_card_id: i32,
    field_energy_to_remove: i32,
}

impl NotifyToOpponentYouUseFieldEnergyRemoveSupportCardRequest {
    pub fn new(opponent_unique_id: i32, usage_hand_card_id: i32, field_energy_to_remove: i32) -> Self {
        NotifyToOpponentYouUseFieldEnergyRemoveSupportCardRequest {
            opponent_unique_id,
            usage_hand_card_id,
            field_energy_to_remove,
        }
    }
    pub fn get_opponent_unique_id(&self) -> i32 { self.opponent_unique_id }
    pub fn get_usage_hand_card_id(&self) -> i32 { self.usage_hand_card_id }
    pub fn get_field_energy_to_remove(&self) -> i32 { self.field_energy_to_remove }
}