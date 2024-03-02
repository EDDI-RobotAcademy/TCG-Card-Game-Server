use std::collections::HashMap;
use async_trait::async_trait;
use crate::common::card_attributes::card_kinds::card_kinds_enum::KindsEnum;
use crate::game_main_character::entity::status_main_character::StatusMainCharacterEnum;
use crate::ui_data_generator::entity::field_unit_damage_info::FieldUnitDamageInfo;
use crate::ui_data_generator::entity::field_unit_energy_info::FieldUnitEnergyInfo;
use crate::ui_data_generator::entity::field_unit_health_point_info::FieldUnitHealthPointInfo;
use crate::ui_data_generator::entity::field_unit_death_info::{FieldUnitDeathInfo};
use crate::ui_data_generator::entity::field_unit_extra_effect_info::FieldUnitExtraEffectInfo;
use crate::ui_data_generator::entity::player_deck_card_lost_list_info::PlayerDeckCardLostListInfo;
use crate::ui_data_generator::entity::player_deck_card_use_list_info::PlayerDeckCardUseListInfo;
use crate::ui_data_generator::entity::player_drawn_card_list_info::PlayerDrawnCardListInfo;
use crate::ui_data_generator::entity::player_field_energy_info::PlayerFieldEnergyInfo;
use crate::ui_data_generator::entity::player_field_unit_damage_info::PlayerFieldUnitDamageInfo;
use crate::ui_data_generator::entity::player_field_unit_death_info::PlayerFieldUnitDeathInfo;
use crate::ui_data_generator::entity::player_field_unit_energy_info::PlayerFieldUnitEnergyInfo;
use crate::ui_data_generator::entity::player_field_unit_health_point_info::PlayerFieldUnitHealthPointInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::ui_data_generator::entity::player_main_character_damage_info::PlayerMainCharacterDamageInfo;
use crate::ui_data_generator::entity::player_main_character_health_point_info::PlayerMainCharacterHealthPointInfo;
use crate::ui_data_generator::entity::player_main_character_survival_info::PlayerMainCharacterSurvivalInfo;
use crate::ui_data_generator::entity::player_search_card_list_info::PlayerSearchCardListInfo;
use crate::ui_data_generator::entity::used_hand_card_info::UsedHandCardInfo;

#[async_trait]
pub trait NotifyPlayerActionInfoRepository {
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
        player_field_energy_map_for_notice: HashMap<PlayerIndex, i32>
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
}