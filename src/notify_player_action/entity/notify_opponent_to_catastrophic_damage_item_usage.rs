use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyOpponentToCatastrophicDamageItemUsage {
    usage_item_card_id: i32,
    damage_for_field_unit: i32,
}

impl NotifyOpponentToCatastrophicDamageItemUsage {
    pub fn new(usage_item_card_id: i32, damage_for_field_unit: i32) -> Self {
        NotifyOpponentToCatastrophicDamageItemUsage {
            usage_item_card_id,
            damage_for_field_unit
        }
    }
    pub fn get_usage_item_card_id(&self) -> i32 { self.usage_item_card_id }
    pub fn get_damage_for_field_unit(&self) -> i32 { self.damage_for_field_unit }
}