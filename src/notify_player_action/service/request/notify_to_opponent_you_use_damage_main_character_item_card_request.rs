#[derive(Debug)]
pub struct NotifyToOpponentYouUseDamageMainCharacterItemCardRequest {
    opponent_unique_id: i32,
    usage_hand_card_id: i32,
    damage_for_main_character: i32,
}

impl NotifyToOpponentYouUseDamageMainCharacterItemCardRequest {
    pub fn new(opponent_unique_id: i32, usage_hand_card_id: i32, damage_for_main_character: i32) -> Self {
        NotifyToOpponentYouUseDamageMainCharacterItemCardRequest {
            opponent_unique_id,
            usage_hand_card_id,
            damage_for_main_character,
        }
    }
    pub fn get_opponent_unique_id(&self) -> i32 { self.opponent_unique_id }
    pub fn get_usage_hand_card_id(&self) -> i32 { self.usage_hand_card_id }
    pub fn get_damage_for_main_character(&self) -> i32 { self.damage_for_main_character }
}