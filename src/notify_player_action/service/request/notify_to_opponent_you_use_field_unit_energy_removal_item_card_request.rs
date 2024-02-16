#[derive(Debug)]
pub struct NotifyOpponentYouUseFieldUnitEnergyRemovalItemCardRequest {
    opponent_unique_id: i32,
    usage_hand_card_id: i32,
    energy_quantity: i32,
}

impl NotifyOpponentYouUseFieldUnitEnergyRemovalItemCardRequest {
    pub fn new(opponent_unique_id: i32, usage_hand_card_id: i32, energy_quantity: i32) -> Self {
        NotifyOpponentYouUseFieldUnitEnergyRemovalItemCardRequest {
            opponent_unique_id,
            usage_hand_card_id,
            energy_quantity,
        }
    }
    pub fn get_opponent_unique_id(&self) -> i32 { self.opponent_unique_id }
    pub fn get_usage_hand_card_id(&self) -> i32 { self.usage_hand_card_id }
    pub fn get_energy_quantity(&self) -> i32 { self.energy_quantity }
}