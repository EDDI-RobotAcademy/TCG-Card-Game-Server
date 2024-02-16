use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyOpponentToDamageMainCharacterItemUsage {
    usage_item_card_id: i32,
    damage_for_main_character: i32,
}

impl NotifyOpponentToDamageMainCharacterItemUsage {
    pub fn new(usage_item_card_id: i32, damage_for_main_character: i32) -> Self {
        NotifyOpponentToDamageMainCharacterItemUsage {
            usage_item_card_id,
            damage_for_main_character
        }
    }
    pub fn get_usage_item_card_id(&self) -> i32 { self.usage_item_card_id }
    pub fn get_damage_for_field_unit(&self) -> i32 { self.damage_for_main_character }
}