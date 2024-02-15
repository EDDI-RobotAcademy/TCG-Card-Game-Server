#[derive(Debug)]
pub struct NotifyToOpponentYouUseCatastrophicDamageItemCardRequest {
    opponent_unique_id: i32,
    usage_hand_card_id: i32,
    damage_for_field_unit: i32,
}

impl NotifyToOpponentYouUseCatastrophicDamageItemCardRequest {
    pub fn new(opponent_unique_id: i32, usage_hand_card_id: i32, damage_for_field_unit: i32) -> Self {
        NotifyToOpponentYouUseCatastrophicDamageItemCardRequest {
            opponent_unique_id,
            usage_hand_card_id,
            damage_for_field_unit,
        }
    }
    pub fn get_opponent_unique_id(&self) -> i32 { self.opponent_unique_id }
    pub fn get_usage_hand_card_id(&self) -> i32 { self.usage_hand_card_id }
    pub fn get_damage_for_field_unit(&self) -> i32 { self.damage_for_field_unit }
}