use std::collections::HashMap;
use async_trait::async_trait;
use crate::game_main_character::entity::status_main_character::StatusMainCharacterEnum;
use crate::ui_data_generator::entity::field_unit_basic_attack_info::FieldUnitAttackInfo;
use crate::ui_data_generator::entity::field_unit_energy_info::FieldUnitEnergyInfo;
use crate::ui_data_generator::entity::field_unit_health_point_info::FieldUnitHealthPointInfo;
use crate::ui_data_generator::entity::field_unit_death_info::{FieldUnitDeathInfo};
use crate::ui_data_generator::entity::field_unit_extra_effect_info::FieldUnitExtraEffectInfo;
use crate::ui_data_generator::entity::field_unit_harmful_status_info::FieldUnitHarmfulStatusInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::ui_data_generator::entity::used_hand_card_info::UsedHandCardInfo;

#[async_trait]
pub trait NotifyPlayerActionInfoRepository {
    async fn notice_deploy_unit(
        &mut self,
        opponent_unique_id: i32,
        player_hand_use_map_for_notice: HashMap<PlayerIndex, UsedHandCardInfo>
    ) -> bool;
    async fn notice_use_field_energy_to_unit(
        &mut self,
        opponent_unique_id: i32,
        player_field_energy_map_for_notice: HashMap<PlayerIndex, i32>,
        player_field_unit_energy_map_for_notice: HashMap<PlayerIndex, FieldUnitEnergyInfo>
    ) -> bool;
    async fn notice_use_general_energy_to_unit(
        &mut self,
        opponent_unique_id: i32,
        player_hand_use_map_for_notice: HashMap<PlayerIndex, UsedHandCardInfo>,
        player_field_unit_energy_map_for_notice: HashMap<PlayerIndex, FieldUnitEnergyInfo>
    ) -> bool;
    async fn notice_use_special_energy_to_unit(
        &mut self,
        opponent_unique_id: i32,
        player_hand_use_map_for_notice: HashMap<PlayerIndex, UsedHandCardInfo>,
        player_field_unit_energy_map_for_notice: HashMap<PlayerIndex, FieldUnitEnergyInfo>,
        player_field_unit_extra_effect_map_for_notice: HashMap<PlayerIndex, FieldUnitExtraEffectInfo>,
    ) -> bool;
    async fn notice_use_unit_energy_boost_support(
        &mut self,
        opponent_unique_id: i32,
        player_hand_use_map_for_notice: HashMap<PlayerIndex, UsedHandCardInfo>,
        player_deck_card_use_list_map_for_notice: HashMap<PlayerIndex, Vec<i32>>,
        player_field_unit_energy_map_for_notice: HashMap<PlayerIndex, FieldUnitEnergyInfo>
    ) -> bool;
    async fn notice_use_draw_support(
        &mut self,
        opponent_unique_id: i32,
        player_hand_use_map_for_notice: HashMap<PlayerIndex, UsedHandCardInfo>,
        player_draw_count_map_for_notice: HashMap<PlayerIndex, i32>,
    ) -> bool;
    async fn notice_use_search_deck_support(
        &mut self,
        opponent_unique_id: i32,
        player_hand_use_map_for_notice: HashMap<PlayerIndex, UsedHandCardInfo>,
        player_search_count_map_for_notice: HashMap<PlayerIndex, i32>,
    ) -> bool;
    async fn notice_use_field_energy_remove_support(
        &mut self,
        opponent_unique_id: i32,
        player_hand_use_map_for_notice: HashMap<PlayerIndex, UsedHandCardInfo>,
        player_field_energy_map_for_notice: HashMap<PlayerIndex, i32>,
    ) -> bool;
    async fn notice_use_instant_unit_death_item(
        &mut self,
        opponent_unique_id: i32,
        player_hand_use_map_for_notice: HashMap<PlayerIndex, UsedHandCardInfo>,
        player_field_unit_health_point_map_for_notice: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
        player_field_unit_death_map_for_notice: HashMap<PlayerIndex, FieldUnitDeathInfo>,
    ) -> bool;
    async fn notice_use_field_energy_increase_item(
        &mut self,
        opponent_unique_id: i32,
        player_hand_use_map_for_notice: HashMap<PlayerIndex, UsedHandCardInfo>,
        player_field_energy_map_for_notice: HashMap<PlayerIndex, i32>,
        player_field_unit_death_map_for_notice: HashMap<PlayerIndex, FieldUnitDeathInfo>,
    ) -> bool;
    async fn notice_use_catastrophic_damage_item(
        &mut self,
        opponent_unique_id: i32,
        player_hand_use_map_for_notice: HashMap<PlayerIndex, UsedHandCardInfo>,
        player_field_unit_health_point_map_for_notice: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
        player_field_unit_death_map_for_notice: HashMap<PlayerIndex, FieldUnitDeathInfo>,
        player_main_character_health_point_map_for_notice: HashMap<PlayerIndex, i32>,
        player_main_character_survival_map_for_notice: HashMap<PlayerIndex, StatusMainCharacterEnum>,
        player_deck_card_lost_list_map_for_notice: HashMap<PlayerIndex, Vec<i32>>,
    ) -> bool;
    async fn notice_use_unit_energy_remove_item(
        &mut self,
        opponent_unique_id: i32,
        player_hand_use_map_for_notice: HashMap<PlayerIndex, UsedHandCardInfo>,
        player_field_unit_energy_map_for_notice: HashMap<PlayerIndex, FieldUnitEnergyInfo>,
        player_field_unit_health_point_map_for_notice: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
        player_field_unit_death_map_for_notice: HashMap<PlayerIndex, FieldUnitDeathInfo>
    ) -> bool;
    async fn notice_use_multiple_unit_damage_item(
        &mut self,
        opponent_unique_id: i32,
        player_hand_use_map_for_notice: HashMap<PlayerIndex, UsedHandCardInfo>,
        player_field_unit_health_point_map_for_notice: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
        player_field_unit_death_map_for_notice: HashMap<PlayerIndex, FieldUnitDeathInfo>
    ) -> bool;
    async fn notice_basic_attack_to_unit(
        &mut self,
        opponent_unique_id: i32,
        player_field_unit_attack_map_for_notice: HashMap<PlayerIndex, FieldUnitAttackInfo>,
        player_field_unit_health_point_map_for_notice: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
        player_field_unit_harmful_effect_map_for_notice: HashMap<PlayerIndex, FieldUnitHarmfulStatusInfo>,
        player_field_unit_death_map_for_notice: HashMap<PlayerIndex, FieldUnitDeathInfo>,
    ) -> bool;
    async fn notice_basic_attack_to_main_character(
        &mut self,
        opponent_unique_id: i32,
        player_field_unit_attack_map_for_notice: HashMap<PlayerIndex, FieldUnitAttackInfo>,
        player_main_character_health_point_map_for_notice: HashMap<PlayerIndex, i32>,
        player_main_character_survival_map_for_notice: HashMap<PlayerIndex, StatusMainCharacterEnum>,
    ) -> bool;
    async fn notice_turn_end(
        &mut self,
        opponent_unique_id: i32,
        player_drawn_card_list_map: HashMap<PlayerIndex, Vec<i32>>,
        player_field_energy_map: HashMap<PlayerIndex, i32>,
        player_field_unit_health_point_map: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
        player_field_unit_harmful_effect_map: HashMap<PlayerIndex, FieldUnitHarmfulStatusInfo>,
        player_field_unit_death_map: HashMap<PlayerIndex, FieldUnitDeathInfo>,
        player_main_character_survival_map: HashMap<PlayerIndex, StatusMainCharacterEnum>,
        unit_index_turn_start_passive_list_map: HashMap<i32, Vec<i32>>,
    ) -> bool;
    async fn notice_targeting_attack_active_skill_to_unit(
        &mut self,
        opponent_unique_id: i32,
        player_field_unit_health_point_map_for_notice: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
        player_field_unit_harmful_effect_map_for_notice: HashMap<PlayerIndex, FieldUnitHarmfulStatusInfo>,
        player_field_unit_death_map_for_notice: HashMap<PlayerIndex, FieldUnitDeathInfo>,
    ) -> bool;
    async fn notice_non_targeting_attack_active_skill(
        &mut self,
        opponent_unique_id: i32,
        player_field_unit_health_point_map_for_notice: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
        player_field_unit_harmful_effect_map_for_notice: HashMap<PlayerIndex, FieldUnitHarmfulStatusInfo>,
        player_field_unit_death_map_for_notice: HashMap<PlayerIndex, FieldUnitDeathInfo>,
    ) -> bool;
    async fn notice_targeting_attack_active_skill_to_game_main_character(
        &mut self,
        opponent_unique_id: i32,
        player_main_character_health_point_map: HashMap<PlayerIndex, i32>,
        player_main_character_survival_map: HashMap<PlayerIndex, StatusMainCharacterEnum>,
    ) -> bool;
    async fn notice_deploy_targeting_attack_passive_skill_to_unit(
        &mut self,
        opponent_unique_id: i32,
        player_field_unit_health_point_map_for_notice: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
        player_field_unit_harmful_effect_map_for_notice: HashMap<PlayerIndex, FieldUnitHarmfulStatusInfo>,
        player_field_unit_death_map_for_notice: HashMap<PlayerIndex, FieldUnitDeathInfo>,
    ) -> bool;
    async fn notice_deploy_non_targeting_attack_passive_skill(
        &mut self,
        opponent_unique_id: i32,
        player_field_unit_health_point_map_for_notice: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
        player_field_unit_harmful_effect_map_for_notice: HashMap<PlayerIndex, FieldUnitHarmfulStatusInfo>,
        player_field_unit_death_map_for_notice: HashMap<PlayerIndex, FieldUnitDeathInfo>,
    ) -> bool;
    async fn notice_deploy_targeting_attack_to_game_main_character(
        &mut self,
        opponent_unique_id: i32,
        player_main_character_health_point_map: HashMap<PlayerIndex, i32>,
        player_main_character_survival_map: HashMap<PlayerIndex, StatusMainCharacterEnum>,
    ) -> bool;
    async fn notice_turn_start_targeting_attack_passive_skill_to_unit(
        &mut self,
        opponent_unique_id: i32,
        player_field_unit_health_point_map_for_notice: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
        player_field_unit_harmful_effect_map_for_notice: HashMap<PlayerIndex, FieldUnitHarmfulStatusInfo>,
        player_field_unit_death_map_for_notice: HashMap<PlayerIndex, FieldUnitDeathInfo>,
    ) -> bool;
    async fn notice_turn_start_non_targeting_attack_passive_skill(
        &mut self,
        opponent_unique_id: i32,
        player_field_unit_health_point_map_for_notice: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
        player_field_unit_harmful_effect_map_for_notice: HashMap<PlayerIndex, FieldUnitHarmfulStatusInfo>,
        player_field_unit_death_map_for_notice: HashMap<PlayerIndex, FieldUnitDeathInfo>,
    ) -> bool;
    async fn notice_turn_start_targeting_attack_to_game_main_character(
        &mut self,
        opponent_unique_id: i32,
        player_main_character_health_point_map: HashMap<PlayerIndex, i32>,
        player_main_character_survival_map: HashMap<PlayerIndex, StatusMainCharacterEnum>,
    ) -> bool;

    // TODO: 도메인 분리를 고민했으면 더 좋았으나 우선 여기에 배치
    async fn notice_mulligan_finished(
        &mut self,
        first_account: i32,
        second_account: i32
    ) -> bool;
}