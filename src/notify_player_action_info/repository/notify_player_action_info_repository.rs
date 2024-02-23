use async_trait::async_trait;
use crate::common::card_attributes::card_kinds::card_kinds_enum::KindsEnum;
use crate::notify_player_action_info::entity::attached_energy_info::AttachedEnergyInfo;
use crate::notify_player_action_info::entity::field_unit_health_point_info::FieldUnitHealthPointInfo;
use crate::notify_player_action_info::entity::field_unit_survival_info::FieldUnitSurvivalInfo;

#[async_trait]
pub trait NotifyPlayerActionInfoRepository {
    async fn notify_player_boost_energy_to_specific_unit_by_using_hand_card(
        &mut self,
        opponent_unique_id: i32,
        used_hand_card_id: i32,
        used_hand_card_type: KindsEnum,
        found_energy_card_id_list_form_deck: Vec<i32>,
        unit_index: i32,
        attached_energy_info: AttachedEnergyInfo) -> bool;
    async fn notify_player_draw_card_by_using_hand_card(
        &mut self,
        opponent_unique_id: i32,
        used_hand_card_id: i32,
        used_hand_card_type: KindsEnum,
        drawn_card_list: Vec<i32>) -> bool;
    async fn notify_player_search_card_by_using_hand_card(
        &mut self,
        opponent_unique_id: i32,
        used_hand_card_id: i32,
        used_hand_card_type: KindsEnum,
        found_card_id_list_from_deck: Vec<i32>) -> bool;
    async fn notify_player_remove_field_energy_by_using_hand_card(
        &mut self,
        opponent_unique_id: i32,
        used_hand_card_id: i32,
        used_hand_card_type: KindsEnum,
        remaining_field_energy_count: i32) -> bool;
    async fn notify_player_remove_energy_of_specific_unit_by_using_hand_card(
        &mut self,
        opponent_unique_id: i32,
        used_hand_card_id: i32,
        used_hand_card_type: KindsEnum,
        opponent_unit_index: i32,
        attached_energy_info: AttachedEnergyInfo) -> bool;
    async fn notify_player_apply_damage_to_unit_by_using_hand_card(
        &mut self,
        opponent_unique_id: i32,
        used_hand_card_id: i32,
        used_hand_card_type: KindsEnum,
        field_unit_health_point_info: FieldUnitHealthPointInfo,
        field_unit_survival_info: FieldUnitSurvivalInfo) -> bool;

}