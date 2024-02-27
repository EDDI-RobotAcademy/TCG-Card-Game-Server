use async_trait::async_trait;
use crate::common::card_attributes::card_kinds::card_kinds_enum::KindsEnum;
use crate::game_main_character::entity::status_main_character::StatusMainCharacterEnum;
use crate::notify_player_action_info::entity::field_unit_damage_info::FieldUnitDamageInfo;
use crate::notify_player_action_info::entity::field_unit_energy_info::FieldUnitEnergyInfo;
use crate::notify_player_action_info::entity::field_unit_health_point_info::FieldUnitHealthPointInfo;
use crate::notify_player_action_info::entity::field_unit_death_info::{FieldUnitDeathInfo};
use crate::notify_player_action_info::entity::player_deck_card_lost_list_info::PlayerDeckCardLostListInfo;
use crate::notify_player_action_info::entity::player_deck_card_use_list_info::PlayerDeckCardUseListInfo;
use crate::notify_player_action_info::entity::player_drawn_card_list_info::PlayerDrawnCardListInfo;
use crate::notify_player_action_info::entity::player_field_energy_info::PlayerFieldEnergyInfo;
use crate::notify_player_action_info::entity::player_field_unit_damage_info::PlayerFieldUnitDamageInfo;
use crate::notify_player_action_info::entity::player_field_unit_death_info::PlayerFieldUnitDeathInfo;
use crate::notify_player_action_info::entity::player_field_unit_energy_info::PlayerFieldUnitEnergyInfo;
use crate::notify_player_action_info::entity::player_field_unit_health_point_info::PlayerFieldUnitHealthPointInfo;
use crate::notify_player_action_info::entity::player_main_character_damage_info::PlayerMainCharacterDamageInfo;
use crate::notify_player_action_info::entity::player_main_character_health_point_info::PlayerMainCharacterHealthPointInfo;
use crate::notify_player_action_info::entity::player_main_character_survival_info::PlayerMainCharacterSurvivalInfo;
use crate::notify_player_action_info::entity::player_search_card_list_info::PlayerSearchCardListInfo;

#[async_trait]
pub trait NotifyPlayerActionInfoRepository {
    async fn notify_player_use_hand_card(
        &mut self,
        opponent_unique_id: i32,
        used_hand_card_id: i32,
        used_hand_card_type: KindsEnum) -> bool;
    async fn notify_player_use_deck_card_list(
        &mut self,
        opponent_unique_id: i32,
        found_card_id_list_form_deck: Vec<i32>) -> PlayerDeckCardUseListInfo;
    async fn notify_player_energy_of_unit(
        &mut self,
        opponent_unique_id: i32,
        field_unit_energy_info: FieldUnitEnergyInfo) -> PlayerFieldUnitEnergyInfo;
    async fn notify_player_draw_card(
        &mut self,
        opponent_unique_id: i32,
        drawn_card_list: Vec<i32>) -> PlayerDrawnCardListInfo;
    async fn notify_player_search_card(
        &mut self,
        opponent_unique_id: i32,
        searched_card_list_from_deck: Vec<i32>) -> PlayerSearchCardListInfo;
    async fn notify_player_field_energy(
        &mut self,
        opponent_unique_id: i32,
        remaining_field_energy_count: i32) -> PlayerFieldEnergyInfo;
    async fn notify_player_opponent_field_energy(
        &mut self,
        opponent_unique_id: i32,
        remaining_field_energy_count: i32) -> PlayerFieldEnergyInfo;
    async fn notify_player_energy_of_specific_opponent_unit(
        &mut self,
        opponent_unique_id: i32,
        field_unit_energy_info: FieldUnitEnergyInfo) -> PlayerFieldUnitEnergyInfo;
    async fn notify_player_apply_damage_to_opponent_unit(
        &mut self,
        opponent_unique_id: i32,
        field_unit_damage_info: FieldUnitDamageInfo,
        field_unit_health_point_info: FieldUnitHealthPointInfo,
        field_unit_death_info: FieldUnitDeathInfo
    ) -> (PlayerFieldUnitDamageInfo, PlayerFieldUnitHealthPointInfo, PlayerFieldUnitDeathInfo);
    async fn notify_player_apply_damage_to_opponent_main_character(
        &mut self,
        opponent_unique_id: i32,
        opponent_main_character_damage: i32,
        opponent_health_point: i32,
        opponent_survival: StatusMainCharacterEnum
    ) -> (PlayerMainCharacterDamageInfo, PlayerMainCharacterHealthPointInfo, PlayerMainCharacterSurvivalInfo);
    async fn notify_player_attach_energy_to_specific_unit(
        &mut self,
        opponent_unique_id: i32,
        field_unit_energy_info: FieldUnitEnergyInfo)-> PlayerFieldUnitEnergyInfo;
    async fn notify_player_death_of_unit(
        &mut self,
        opponent_unique_id: i32,
        field_unit_death_info: FieldUnitDeathInfo) -> PlayerFieldUnitDeathInfo;
    async fn notify_player_death_of_opponent_unit(
        &mut self,
        opponent_unique_id: i32,
        field_unit_death_info: FieldUnitDeathInfo) -> PlayerFieldUnitDeathInfo;
    async fn notify_player_lost_deck_card(
        &mut self,
        opponent_unique_id: i32,
        lost_deck_card_list: Vec<i32>) -> PlayerDeckCardLostListInfo;
}