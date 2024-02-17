use async_trait::async_trait;
use crate::notify_player_action::service::request::notify_to_opponent_you_use_catastrophic_damage_item_card_request::NotifyToOpponentYouUseCatastrophicDamageItemCardRequest;
use crate::notify_player_action::service::response::notify_to_opponent_you_use_catastrophic_damage_item_card_response::NotifyToOpponentYouUseCatastrophicDamageItemCardResponse;

#[async_trait]
pub trait NotifyPlayerActionRepository {
    async fn notify_to_opponent_what_you_do(&mut self, opponent_unique_id: i32, unit_card_number: i32) -> bool;
    async fn notify_to_opponent_you_use_energy_card(&mut self, opponent_unique_id: i32, usage_energy_card_id: i32, unit_card_index: i32, energy_race: i32, energy_count: i32) -> bool;
    async fn notify_to_opponent_you_use_energy_boost_card(&mut self, opponent_unique_id: i32, unit_card_index: i32, usage_support_card_id: i32, boosting_energy_count: i32, boosting_energy_card_id: i32) -> bool;
    async fn notify_to_opponent_you_use_item_instant_death_card(&mut self, opponent_unique_id: i32, unit_card_index: i32, usage_item_card_id: i32) -> bool;
    async fn notify_to_opponent_you_use_item_instant_death_card_alternatives(&mut self, opponent_unique_id: i32, unit_card_index: i32, usage_item_card_id: i32, alternatives_damage: i32) -> bool;
    async fn notify_to_opponent_you_use_draw_support_card(&mut self, opponent_unique_id: i32, usage_support_card_id: i32, draw_card_count: i32) -> bool;
    async fn notify_opponent_you_use_search_support_card(&mut self, opponent_unique_id: i32, usage_support_card_id: i32, found_card_count: i32) -> bool;
    async fn notify_opponent_you_use_remove_field_energy_support_card(&mut self, opponent_unique_id: i32, usage_support_card_id: i32, amount_to_remove: i32) -> bool;
    async fn notify_opponent_you_use_item_field_energy_increase_card(&mut self, opponent_unique_id: i32, usage_item_card_id: i32, increased_field_energy: i32) -> bool;
    async fn notify_to_opponent_you_use_tool_card_to_enhance_attack_point(&mut self, opponent_unique_id: i32, unit_card_index: i32, usage_item_card_id: i32) -> bool;
    async fn notify_to_opponent_you_use_catastrophic_damage_item_card(&mut self,opponent_unique_id: i32, usage_item_card_id: i32, damage_for_field_unit: i32) -> bool;
    async fn notify_to_opponent_you_use_damage_main_character_item_card(&mut self,opponent_unique_id: i32, usage_item_card_id: i32, damage_for_main_character: i32) -> bool;
    async fn notify_to_opponent_you_use_destroy_deck_item_card(&mut self,opponent_unique_id: i32, usage_item_card_id: i32, will_be_lost_card: i32) -> bool;
    async fn notify_to_opponent_you_use_field_unit_energy_removal_item_card(&mut self,opponent_unique_id: i32, usage_item_card_id: i32, energy_quantity: i32) -> bool;
}