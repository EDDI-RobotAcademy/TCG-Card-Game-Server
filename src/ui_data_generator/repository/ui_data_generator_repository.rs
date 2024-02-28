use async_trait::async_trait;
use crate::common::card_attributes::card_kinds::card_kinds_enum::KindsEnum;
use crate::game_field_unit::entity::attached_energy_map::AttachedEnergyMap;
use crate::ui_data_generator::entity::player_deck_card_use_list_info::PlayerDeckCardUseListInfo;
use crate::ui_data_generator::entity::player_draw_count_info::PlayerDrawCountInfo;
use crate::ui_data_generator::entity::player_drawn_card_list_info::PlayerDrawnCardListInfo;
use crate::ui_data_generator::entity::player_field_energy_info::PlayerFieldEnergyInfo;
use crate::ui_data_generator::entity::player_field_unit_death_info::PlayerFieldUnitDeathInfo;
use crate::ui_data_generator::entity::player_field_unit_energy_info::PlayerFieldUnitEnergyInfo;
use crate::ui_data_generator::entity::player_hand_card_use_info::PlayerHandCardUseInfo;
use crate::ui_data_generator::entity::player_search_card_list_info::PlayerSearchCardListInfo;
use crate::ui_data_generator::entity::player_search_count_info::PlayerSearchCountInfo;

#[async_trait]
pub trait UiDataGeneratorRepository {
    async fn generate_use_my_hand_card_data(
        &mut self,
        used_hand_card_id: i32,
        used_hand_card_kind: KindsEnum
    ) -> (bool,
          PlayerHandCardUseInfo);
    async fn generate_my_specific_unit_energy_data(
        &mut self,
        unit_index: i32,
        updated_unit_energy_map: AttachedEnergyMap
    ) -> (PlayerFieldUnitEnergyInfo,
          PlayerFieldUnitEnergyInfo);
    async fn generate_use_my_field_energy_data(
        &mut self,
        remaining_field_energy: i32
    ) -> (PlayerFieldEnergyInfo,
          PlayerFieldEnergyInfo);
    async fn generate_use_support_card_to_boost_energy_to_my_specific_unit_data(
        &mut self,
        used_hand_card_id: i32,
        used_hand_card_kind: KindsEnum,
        found_energy_card_id_list: Vec<i32>,
        unit_index: i32,
        updated_unit_energy_map: AttachedEnergyMap
    ) -> (PlayerDeckCardUseListInfo,
          PlayerFieldUnitEnergyInfo,
          PlayerHandCardUseInfo,
          PlayerDeckCardUseListInfo,
          PlayerFieldUnitEnergyInfo);
    async fn generate_use_support_card_to_draw_my_deck_data(
        &mut self,
        used_hand_card_id: i32,
        used_hand_card_kind: KindsEnum,
        drawn_card_list: Vec<i32>
    ) -> (PlayerDrawnCardListInfo,
          PlayerHandCardUseInfo,
          PlayerDrawCountInfo);
    async fn generate_use_support_card_to_search_unit_from_my_deck_data(
        &mut self,
        used_hand_card_id: i32,
        used_hand_card_kind: KindsEnum,
        found_card_list: Vec<i32>
    ) -> (PlayerSearchCardListInfo,
          PlayerHandCardUseInfo,
          PlayerSearchCountInfo);
    async fn generate_use_support_card_to_remove_your_field_energy_data(
        &mut self,
        used_hand_card_id: i32,
        used_hand_card_kind: KindsEnum,
        remaining_field_energy: i32
    ) -> (PlayerFieldEnergyInfo,
          PlayerHandCardUseInfo,
          PlayerFieldEnergyInfo);
    async fn generate_instant_death_of_your_specific_unit_data(
        &mut self,
        dead_unit_index: i32
    ) -> (PlayerFieldUnitDeathInfo,
          PlayerFieldUnitDeathInfo);
}