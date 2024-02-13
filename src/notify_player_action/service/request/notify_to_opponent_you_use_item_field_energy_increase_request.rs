#[derive(Debug)]
pub struct NotifyOpponentYouUseItemFieldEnergyIncreaseRequest {
    opponent_unique_id: i32,
    usage_item_card_id: i32,
    increased_field_energy: i32,
}

impl NotifyOpponentYouUseItemFieldEnergyIncreaseRequest {
    pub fn new(opponent_unique_id: i32,
               usage_item_card_id: i32,
               increased_field_energy: i32) -> Self {
        NotifyOpponentYouUseItemFieldEnergyIncreaseRequest {
            opponent_unique_id,
            usage_item_card_id,
            increased_field_energy,
        }
    }

    pub fn get_opponent_unique_id(&self) -> i32 { self.opponent_unique_id }
    pub fn get_usage_item_card_id(&self) -> i32 { self.usage_item_card_id }
    pub fn get_increased_field_energy(&self) -> i32 { self.increased_field_energy }
}