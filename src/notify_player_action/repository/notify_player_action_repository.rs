use async_trait::async_trait;

#[async_trait]
pub trait NotifyPlayerActionRepository {
    async fn notify_to_opponent_what_you_do(&mut self, opponent_unique_id: i32, unit_card_number: i32) -> bool;
    async fn notify_to_opponent_you_use_energy_card(&mut self, opponent_unique_id: i32, unit_card_index: i32, usage_energy_card_id: i32) -> bool;
    async fn notify_to_opponent_you_use_energy_boost_card(&mut self, opponent_unique_id: i32, unit_card_index: i32, usage_support_card_id: i32, boosting_energy_count: i32, boosting_energy_card_id: i32) -> bool;
    async fn notify_to_opponent_you_use_item_instant_death_card(&mut self, opponent_unique_id: i32, unit_card_index: i32, usage_item_card_id: i32) -> bool;
    async fn notify_to_opponent_you_use_item_instant_death_card_alternatives(&mut self, opponent_unique_id: i32, unit_card_index: i32, usage_item_card_id: i32, alternatives_damage: i32) -> bool;
    async fn notify_to_opponent_you_use_draw_support_card(&mut self, opponent_unique_id: i32, usage_support_card_id: i32, draw_card_count: i32) -> bool;
    async fn notify_opponent_you_use_search_support_card(&mut self, opponent_unique_id: i32, usage_support_card_id: i32, found_card_count: i32) -> bool;
    async fn notify_opponent_you_use_remove_field_energy_support_card(&mut self, opponent_unique_id: i32, usage_support_card_id: i32, amount_to_remove: i32) -> bool;
}