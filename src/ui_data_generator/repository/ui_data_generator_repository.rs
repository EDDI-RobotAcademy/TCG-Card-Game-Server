use async_trait::async_trait;
use crate::common::card_attributes::card_kinds::card_kinds_enum::KindsEnum;
use crate::game_field_unit::entity::attached_energy_map::AttachedEnergyMap;
use crate::game_field_unit::entity::extra_effect::ExtraEffect;
use crate::game_main_character::entity::status_main_character::StatusMainCharacterEnum;
use crate::ui_data_generator::entity::player_deck_card_lost_list_info::PlayerDeckCardLostListInfo;
use crate::ui_data_generator::entity::player_deck_card_use_list_info::PlayerDeckCardUseListInfo;
use crate::ui_data_generator::entity::player_draw_count_info::PlayerDrawCountInfo;
use crate::ui_data_generator::entity::player_drawn_card_list_info::PlayerDrawnCardListInfo;
use crate::ui_data_generator::entity::player_field_energy_info::PlayerFieldEnergyInfo;
use crate::ui_data_generator::entity::player_field_unit_basic_attack_info::PlayerFieldUnitAttackInfo;
use crate::ui_data_generator::entity::player_field_unit_death_info::PlayerFieldUnitDeathInfo;
use crate::ui_data_generator::entity::player_field_unit_energy_info::PlayerFieldUnitEnergyInfo;
use crate::ui_data_generator::entity::player_field_unit_extra_effect_info::PlayerFieldUnitExtraEffectInfo;
use crate::ui_data_generator::entity::player_field_unit_harmful_effect_info::PlayerFieldUnitHarmfulEffectInfo;
use crate::ui_data_generator::entity::player_field_unit_health_point_info::PlayerFieldUnitHealthPointInfo;
use crate::ui_data_generator::entity::player_hand_card_use_info::PlayerHandCardUseInfo;
use crate::ui_data_generator::entity::player_main_character_health_point_info::PlayerMainCharacterHealthPointInfo;
use crate::ui_data_generator::entity::player_main_character_survival_info::PlayerMainCharacterSurvivalInfo;
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
    async fn generate_my_specific_unit_health_point_data(
        &mut self,
        my_unit_index: i32,
        my_unit_updated_health_point: i32
    ) -> (PlayerFieldUnitHealthPointInfo,
          PlayerFieldUnitHealthPointInfo);
    async fn generate_my_main_character_health_point_data(
        &mut self,
        my_main_character_updated_health_point: i32
    ) -> (PlayerMainCharacterHealthPointInfo,
          PlayerMainCharacterHealthPointInfo);
    async fn generate_opponent_main_character_health_point_data(
        &mut self,
        opponent_main_character_updated_health_point: i32
    ) -> (PlayerMainCharacterHealthPointInfo,
          PlayerMainCharacterHealthPointInfo);
    async fn generate_my_multiple_unit_health_point_data(
        &mut self,
        opponent_unit_health_point_tuple_list: Vec<(i32, i32)>
    ) -> (PlayerFieldUnitHealthPointInfo,
          PlayerFieldUnitHealthPointInfo);
    async fn generate_my_specific_unit_energy_data(
        &mut self,
        unit_index: i32,
        updated_unit_energy_map: AttachedEnergyMap
    ) -> (PlayerFieldUnitEnergyInfo,
          PlayerFieldUnitEnergyInfo);
    async fn generate_opponent_specific_unit_energy_data(
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
    async fn generate_use_my_deck_card_list_data(
        &mut self,
        deck_card_id_list: Vec<i32>,
    ) -> (PlayerDeckCardUseListInfo,
          PlayerDeckCardUseListInfo);
    async fn generate_draw_my_deck_data(
        &mut self,
        drawn_card_list: Vec<i32>
    ) -> (PlayerDrawnCardListInfo,
          PlayerDrawCountInfo);
    async fn generate_draw_opponent_deck_data(
        &mut self,
        drawn_card_list: Vec<i32>
    ) -> (PlayerDrawCountInfo,
          PlayerDrawnCardListInfo
          );
    async fn generate_search_my_deck_data(
        &mut self,
        found_card_list: Vec<i32>
    ) -> (PlayerSearchCardListInfo,
          PlayerSearchCountInfo);
    async fn generate_opponent_field_energy_data(
        &mut self,
        remaining_field_energy: i32
    ) -> (PlayerFieldEnergyInfo,
          PlayerFieldEnergyInfo);
    async fn generate_opponent_specific_unit_death_data(
        &mut self,
        dead_unit_index: i32
    ) -> (PlayerFieldUnitDeathInfo,
          PlayerFieldUnitDeathInfo);
    async fn generate_opponent_multiple_unit_death_data(
        &mut self,
        dead_unit_index_list: Vec<i32>
    ) -> (PlayerFieldUnitDeathInfo,
          PlayerFieldUnitDeathInfo);
    async fn generate_my_multiple_unit_death_data(
        &mut self,
        dead_unit_index_list: Vec<i32>
    ) -> (PlayerFieldUnitDeathInfo,
          PlayerFieldUnitDeathInfo);
    async fn generate_my_specific_unit_death_data(
        &mut self,
        dead_unit_index: i32
    ) -> (PlayerFieldUnitDeathInfo,
          PlayerFieldUnitDeathInfo);

    async fn generate_opponent_specific_unit_health_point_data(
        &mut self,
        opponent_unit_index: i32,
        opponent_unit_updated_health_point: i32
    ) -> (PlayerFieldUnitHealthPointInfo,
          PlayerFieldUnitHealthPointInfo);
    async fn generate_opponent_multiple_unit_health_point_data(
        &mut self,
        opponent_unit_health_point_tuple_list: Vec<(i32, i32)>
    ) -> (PlayerFieldUnitHealthPointInfo,
          PlayerFieldUnitHealthPointInfo);
    async fn generate_my_main_character_survival_data(
        &mut self,
        my_main_character_status: StatusMainCharacterEnum
    ) -> (PlayerMainCharacterSurvivalInfo,
          PlayerMainCharacterSurvivalInfo);
    async fn generate_opponent_main_character_survival_data(
        &mut self,
        my_main_character_status: StatusMainCharacterEnum
    ) -> (PlayerMainCharacterSurvivalInfo,
          PlayerMainCharacterSurvivalInfo);
    async fn generate_my_specific_unit_extra_effect_data(
        &mut self,
        my_extra_effect_unit_index:i32,
        my_unit_extra_effect_list:Vec<ExtraEffect>
    ) -> (PlayerFieldUnitExtraEffectInfo,
          PlayerFieldUnitExtraEffectInfo);
    async fn generate_opponent_specific_unit_extra_effect_data(
        &mut self,
        opponent_extra_effect_unit_index:i32,
        opponent_unit_extra_effect_list:Vec<ExtraEffect>
    ) -> (PlayerFieldUnitExtraEffectInfo,
          PlayerFieldUnitExtraEffectInfo);
    async fn generate_my_specific_unit_harmful_effect_data(
        &mut self,
        my_harmful_effect_unit_index:i32,
        my_unit_harmful_effect_list:Vec<ExtraEffect>
    ) -> (PlayerFieldUnitHarmfulEffectInfo,
          PlayerFieldUnitHarmfulEffectInfo);
    async fn generate_opponent_specific_unit_harmful_effect_data(
        &mut self,
        opponent_harmful_effect_unit_index:i32,
        opponent_unit_harmful_effect_list:Vec<ExtraEffect>
    ) -> (PlayerFieldUnitHarmfulEffectInfo,
          PlayerFieldUnitHarmfulEffectInfo);
    async fn generate_my_multiple_unit_harmful_effect_data(
        &mut self,
        my_unit_harmful_effect_tuple_list: Vec<(i32, Vec<ExtraEffect>)>
    ) -> (PlayerFieldUnitHarmfulEffectInfo,
          PlayerFieldUnitHarmfulEffectInfo);
    async fn generate_opponent_multiple_unit_harmful_effect_data(
        &mut self,
        opponent_unit_harmful_effect_tuple_list: Vec<(i32, Vec<ExtraEffect>)>
    ) -> (PlayerFieldUnitHarmfulEffectInfo,
          PlayerFieldUnitHarmfulEffectInfo);
    async fn generate_my_multiple_unit_extra_effect_data(
        &mut self,
        my_unit_extra_effect_tuple_list: Vec<(i32, Vec<ExtraEffect>)>
    ) -> (PlayerFieldUnitExtraEffectInfo,
          PlayerFieldUnitExtraEffectInfo);
    async fn generate_opponent_multiple_unit_extra_effect_data(
        &mut self,
        opponent_unit_extra_effect_tuple_list: Vec<(i32, Vec<ExtraEffect>)>
    ) -> (PlayerFieldUnitExtraEffectInfo,
          PlayerFieldUnitExtraEffectInfo);
    async fn generate_opponent_deck_card_lost_data(
        &mut self,
        opponent_lost_deck_card_list: Vec<i32>
    ) -> (PlayerDeckCardLostListInfo,
          PlayerDeckCardLostListInfo);
    async fn generate_my_specific_unit_basic_attack_data(
        &mut self,
        attacker_unit_index: i32,
        target_unit_index: i32,
    ) -> (PlayerFieldUnitAttackInfo,
          PlayerFieldUnitAttackInfo);
    async fn generate_my_specific_unit_active_skill_use_data(
        &mut self,
        attacker_unit_index: i32,
        target_unit_index: i32,
        active_skill_index: i32,
    ) -> (PlayerFieldUnitAttackInfo,
          PlayerFieldUnitAttackInfo);
    async fn generate_my_specific_unit_passive_skill_use_data(
        &mut self,
        attacker_unit_index: i32,
        target_unit_index: i32,
        passive_skill_index: i32,
    ) -> (PlayerFieldUnitAttackInfo,
          PlayerFieldUnitAttackInfo);
}