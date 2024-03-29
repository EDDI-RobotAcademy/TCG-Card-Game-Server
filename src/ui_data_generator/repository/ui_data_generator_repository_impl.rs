use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::common::card_attributes::card_kinds::card_kinds_enum::KindsEnum;
use crate::game_field_unit::entity::attached_energy_map::AttachedEnergyMap;
use crate::game_field_unit::entity::extra_effect::ExtraEffect;
use crate::game_main_character::entity::status_main_character::StatusMainCharacterEnum;
use crate::ui_data_generator::entity::basic_attack_info::AttackInfo;
use crate::ui_data_generator::entity::extra_effect_info::ExtraEffectInfo;
use crate::ui_data_generator::entity::field_unit_basic_attack_info::FieldUnitAttackInfo;
use crate::ui_data_generator::entity::field_unit_damage_info::FieldUnitDamageInfo;
use crate::ui_data_generator::entity::field_unit_death_info::FieldUnitDeathInfo;
use crate::ui_data_generator::entity::field_unit_energy_info::FieldUnitEnergyInfo;
use crate::ui_data_generator::entity::field_unit_extra_effect_info::FieldUnitExtraEffectInfo;
use crate::ui_data_generator::entity::field_unit_harmful_status_info::FieldUnitHarmfulStatusInfo;
use crate::ui_data_generator::entity::field_unit_health_point_info::FieldUnitHealthPointInfo;
use crate::ui_data_generator::entity::harmful_effect_info::HarmfulStatusInfo;
use crate::ui_data_generator::entity::player_deck_card_lost_list_info::PlayerDeckCardLostListInfo;
use crate::ui_data_generator::entity::player_deck_card_use_list_info::PlayerDeckCardUseListInfo;
use crate::ui_data_generator::entity::player_draw_count_info::PlayerDrawCountInfo;
use crate::ui_data_generator::entity::player_drawn_card_list_info::PlayerDrawnCardListInfo;
use crate::ui_data_generator::entity::player_field_energy_info::PlayerFieldEnergyInfo;
use crate::ui_data_generator::entity::player_field_unit_basic_attack_info::PlayerFieldUnitAttackInfo;
use crate::ui_data_generator::entity::player_field_unit_damage_info::PlayerFieldUnitDamageInfo;
use crate::ui_data_generator::entity::player_field_unit_death_info::PlayerFieldUnitDeathInfo;
use crate::ui_data_generator::entity::player_field_unit_energy_info::PlayerFieldUnitEnergyInfo;
use crate::ui_data_generator::entity::player_field_unit_extra_effect_info::PlayerFieldUnitExtraEffectInfo;
use crate::ui_data_generator::entity::player_field_unit_harmful_effect_info::PlayerFieldUnitHarmfulEffectInfo;
use crate::ui_data_generator::entity::player_field_unit_health_point_info::PlayerFieldUnitHealthPointInfo;
use crate::ui_data_generator::entity::player_hand_card_use_info::PlayerHandCardUseInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex::{Opponent, You};
use crate::ui_data_generator::entity::player_main_character_damage_info::PlayerMainCharacterDamageInfo;
use crate::ui_data_generator::entity::player_main_character_health_point_info::PlayerMainCharacterHealthPointInfo;
use crate::ui_data_generator::entity::player_main_character_survival_info::PlayerMainCharacterSurvivalInfo;
use crate::ui_data_generator::entity::player_search_card_list_info::PlayerSearchCardListInfo;
use crate::ui_data_generator::entity::player_search_count_info::PlayerSearchCountInfo;
use crate::ui_data_generator::entity::used_hand_card_info::UsedHandCardInfo;
use crate::ui_data_generator::repository::ui_data_generator_repository::UiDataGeneratorRepository;

pub struct UiDataGeneratorRepositoryImpl;

impl UiDataGeneratorRepositoryImpl {
    pub fn new() -> Self {
        UiDataGeneratorRepositoryImpl {}
    }

    pub fn get_instance() -> Arc<AsyncMutex<UiDataGeneratorRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<UiDataGeneratorRepositoryImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        UiDataGeneratorRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }

    fn get_field_unit_energy_info(
        &self,
        unit_index: i32,
        attached_energy_map: AttachedEnergyMap
    ) -> FieldUnitEnergyInfo {

        let mut map = HashMap::new();
        map.insert(unit_index, attached_energy_map.to_attached_energy_info());

        FieldUnitEnergyInfo::new(map)
    }

    fn get_field_unit_basic_attack_info(
        &self,
        attacker_unit_index: i32,
        target_player_index: PlayerIndex,
        target_unit_index: i32) -> FieldUnitAttackInfo {

        let mut map = HashMap::new();
        map.insert(attacker_unit_index,
                   AttackInfo::new(target_player_index,
                                   target_unit_index,
                                   -1,
                                   -1));

        FieldUnitAttackInfo::new(map)
    }

    fn get_field_unit_active_skill_attack_info(
        &self,
        attacker_unit_index: i32,
        target_player_index: PlayerIndex,
        target_unit_index: i32,
        active_skill_index: i32) -> FieldUnitAttackInfo {

        let mut map = HashMap::new();
        map.insert(attacker_unit_index,
                   AttackInfo::new(target_player_index,
                                   target_unit_index,
                                   active_skill_index,
                                   -1));

        FieldUnitAttackInfo::new(map)
    }

    fn get_field_unit_passive_attack_to_unit_info(
        &self,
        attacker_unit_index: i32,
        target_player_index: PlayerIndex,
        target_unit_index: i32,
        passive_skill_index: i32) -> FieldUnitAttackInfo {

        let mut map = HashMap::new();
        map.insert(attacker_unit_index,
                   AttackInfo::new(target_player_index,
                                   target_unit_index,
                                   -1,
                                   passive_skill_index));

        FieldUnitAttackInfo::new(map)
    }

    fn get_field_unit_death_info(
        &self,
        unit_index: i32,) -> FieldUnitDeathInfo {

        let mut list = Vec::new();
        if unit_index != -1 {
            list.push(unit_index);
        }

        FieldUnitDeathInfo::new(list)
    }

    fn get_field_unit_death_info_by_list(
        &self,
        dead_unit_index_list: Vec<i32>) -> FieldUnitDeathInfo {
        let mut dead_unit_index_list_mut = dead_unit_index_list.clone();
        dead_unit_index_list_mut.retain(|index| index != &-1);

        FieldUnitDeathInfo::new(dead_unit_index_list_mut)
    }

    fn get_field_unit_health_info(
        &self,
        unit_index: i32,
        health_point: i32) -> FieldUnitHealthPointInfo {

        let mut map = HashMap::new();
        map.insert(unit_index, health_point);

        FieldUnitHealthPointInfo::new(map)
    }

    fn get_field_unit_health_info_from_tuple_list(
        &self,
        unit_health_point_tuple_list: Vec<(i32, i32)>) -> FieldUnitHealthPointInfo {

        let mut map = HashMap::new();
        for (unit_index, health_point) in unit_health_point_tuple_list {
            map.insert(unit_index, health_point);
        }

        FieldUnitHealthPointInfo::new(map)
    }

    fn get_field_unit_extra_effect_from_tuple_list(
        &self,
        unit_extra_effect_tuple_list: Vec<(i32, Vec<ExtraEffect>)>) ->FieldUnitExtraEffectInfo {

        let mut map = HashMap::new();
        for (unit_index, extra_effect_list) in unit_extra_effect_tuple_list {
            let extra_effect_info = ExtraEffectInfo::new(extra_effect_list);
            map.insert(unit_index, extra_effect_info);
        }

        FieldUnitExtraEffectInfo::new(map)
    }
    fn get_field_unit_harmful_effect_from_tuple_list(
        &self,
        unit_harmful_effect_tuple_list: Vec<(i32, Vec<ExtraEffect>)>) ->FieldUnitHarmfulStatusInfo {

        let mut map = HashMap::new();
        for (unit_index, harmful_effect_list) in unit_harmful_effect_tuple_list {
            let extra_effect_info = HarmfulStatusInfo::new(harmful_effect_list);
            map.insert(unit_index, extra_effect_info);
        }

        FieldUnitHarmfulStatusInfo::new(map)
    }

    fn get_player_hand_card_use_info(&self,
                                     notify_player_index: PlayerIndex,
                                     used_card_id: i32,
                                     used_card_type: KindsEnum
    ) -> PlayerHandCardUseInfo {

        if used_card_type == KindsEnum::Trap {
            let used_card_info = UsedHandCardInfo::new(-1, used_card_type as i32);
            let mut player_hand_use_map = HashMap::new();
            player_hand_use_map.insert(notify_player_index, used_card_info);

            return PlayerHandCardUseInfo::new(player_hand_use_map)
        }

        let used_card_info = UsedHandCardInfo::new(used_card_id, used_card_type as i32);
        let mut player_hand_use_map = HashMap::new();
        player_hand_use_map.insert(notify_player_index, used_card_info);

        PlayerHandCardUseInfo::new(player_hand_use_map)
    }

    fn get_player_deck_card_list_use_info(&self,
                                          notify_player_index: PlayerIndex,
                                          used_deck_card_list: Vec<i32>
    ) -> PlayerDeckCardUseListInfo {

        let mut player_deck_card_list_use_map = HashMap::new();
        player_deck_card_list_use_map.insert(notify_player_index, used_deck_card_list);

        PlayerDeckCardUseListInfo::new(player_deck_card_list_use_map)
    }

    fn get_player_field_unit_energy_info(&self,
                                         notify_player_index: PlayerIndex,
                                         field_unit_energy_info: FieldUnitEnergyInfo
    ) -> PlayerFieldUnitEnergyInfo {

        let mut player_field_unit_energy_map = HashMap::new();
        player_field_unit_energy_map.insert(notify_player_index, field_unit_energy_info);

        PlayerFieldUnitEnergyInfo::new(player_field_unit_energy_map)
    }

    fn get_player_drawn_card_list_info(&self,
                                       notify_player_index: PlayerIndex,
                                       drawn_card_list: Vec<i32>
    ) -> PlayerDrawnCardListInfo {

        let mut player_drawn_card_list_map = HashMap::new();
        player_drawn_card_list_map.insert(notify_player_index, drawn_card_list);

        PlayerDrawnCardListInfo::new(player_drawn_card_list_map)
    }

    fn get_player_draw_count_info(&self,
                                  notify_player_index: PlayerIndex,
                                  drawn_card_list: Vec<i32>
    ) -> PlayerDrawCountInfo {

        let mut player_draw_count_map = HashMap::new();
        player_draw_count_map.insert(notify_player_index, drawn_card_list.len() as i32);

        PlayerDrawCountInfo::new(player_draw_count_map)
    }

    fn get_player_search_card_list_info(&self,
                                        notify_player_index: PlayerIndex,
                                        found_card_list: Vec<i32>
    ) -> PlayerSearchCardListInfo {

        let mut player_search_card_list_map = HashMap::new();
        player_search_card_list_map.insert(notify_player_index, found_card_list);

        PlayerSearchCardListInfo::new(player_search_card_list_map)
    }

    fn get_player_search_count_info(&self,
                                    notify_player_index: PlayerIndex,
                                    found_card_list: Vec<i32>
    ) -> PlayerSearchCountInfo {

        let mut player_search_count_map = HashMap::new();
        player_search_count_map.insert(notify_player_index, found_card_list.len() as i32);

        PlayerSearchCountInfo::new(player_search_count_map)
    }

    fn get_player_field_energy_info(&self,
                                    notify_player_index: PlayerIndex,
                                    field_energy_count: i32
    ) -> PlayerFieldEnergyInfo {

        let mut player_field_energy_map = HashMap::new();
        player_field_energy_map.insert(notify_player_index, field_energy_count);

        PlayerFieldEnergyInfo::new(player_field_energy_map)
    }

    fn get_player_field_unit_attack_info(&self,
                                         notify_player_index: PlayerIndex,
                                         field_unit_attack_info: FieldUnitAttackInfo
    ) -> PlayerFieldUnitAttackInfo {

        let mut player_field_unit_attack_map = HashMap::new();
        player_field_unit_attack_map.insert(notify_player_index, field_unit_attack_info);

        PlayerFieldUnitAttackInfo::new(player_field_unit_attack_map)
    }

    fn get_player_field_unit_damage_info(&self,
                                         notify_player_index: PlayerIndex,
                                         field_unit_damage_info: FieldUnitDamageInfo
    ) -> PlayerFieldUnitDamageInfo {

        let mut player_field_unit_damage_map = HashMap::new();
        player_field_unit_damage_map.insert(notify_player_index, field_unit_damage_info);

        PlayerFieldUnitDamageInfo::new(player_field_unit_damage_map)
    }

    fn get_player_field_unit_health_point_info(&self,
                                               notify_player_index: PlayerIndex,
                                               field_unit_health_point_info: FieldUnitHealthPointInfo
    ) -> PlayerFieldUnitHealthPointInfo {

        let mut player_field_unit_health_point_map = HashMap::new();
        player_field_unit_health_point_map.insert(notify_player_index, field_unit_health_point_info);

        PlayerFieldUnitHealthPointInfo::new(player_field_unit_health_point_map)
    }

    fn get_player_field_unit_death_info(&self,
                                        notify_player_index: PlayerIndex,
                                        field_unit_death_info: FieldUnitDeathInfo
    ) -> PlayerFieldUnitDeathInfo {

        let mut player_field_unit_death_map = HashMap::new();
        player_field_unit_death_map.insert(notify_player_index, field_unit_death_info);

        PlayerFieldUnitDeathInfo::new(player_field_unit_death_map)
    }

    fn get_player_deck_card_lost_list_info(&self,
                                           notify_player_index: PlayerIndex,
                                           lost_deck_card_list: Vec<i32>
    ) -> PlayerDeckCardLostListInfo {

        let mut player_deck_card_lost_list_map = HashMap::new();
        player_deck_card_lost_list_map.insert(notify_player_index, lost_deck_card_list);

        PlayerDeckCardLostListInfo::new(player_deck_card_lost_list_map)
    }

    fn get_player_main_character_damage_info(&self,
                                             notify_player_index: PlayerIndex,
                                             damage: i32
    ) -> PlayerMainCharacterDamageInfo {

        let mut player_main_character_damage_map = HashMap::new();
        player_main_character_damage_map.insert(notify_player_index, damage);

        PlayerMainCharacterDamageInfo::new(player_main_character_damage_map)
    }

    fn get_player_main_character_health_point_info(&self,
                                                   notify_player_index: PlayerIndex,
                                                   health_point: i32
    ) -> PlayerMainCharacterHealthPointInfo {

        let mut player_main_character_health_point_map = HashMap::new();
        player_main_character_health_point_map.insert(notify_player_index, health_point);

        PlayerMainCharacterHealthPointInfo::new(player_main_character_health_point_map)
    }

    fn get_player_main_character_survival_info(&self,
                                               notify_player_index: PlayerIndex,
                                               survival_status: StatusMainCharacterEnum
    ) -> PlayerMainCharacterSurvivalInfo {

        let mut player_main_character_survival_map = HashMap::new();
        player_main_character_survival_map.insert(notify_player_index, survival_status);

        PlayerMainCharacterSurvivalInfo::new(player_main_character_survival_map)
    }
    fn get_field_unit_extra_effect_info(
        &self,
        unit_index: i32,
        unit_extra_effect_list: Vec<ExtraEffect>) -> FieldUnitExtraEffectInfo {

        let extra_effect_info = ExtraEffectInfo::new(unit_extra_effect_list);

        let mut map = HashMap::new();
        map.insert(unit_index, extra_effect_info);

        FieldUnitExtraEffectInfo::new(map)
    }
    fn get_field_unit_harmful_effect_info(
        &self,
        unit_index: i32,
        unit_harmful_effect_list: Vec<ExtraEffect>) -> FieldUnitHarmfulStatusInfo {

        let extra_effect_info = HarmfulStatusInfo::new(unit_harmful_effect_list);

        let mut map = HashMap::new();
        map.insert(unit_index, extra_effect_info);

        FieldUnitHarmfulStatusInfo::new(map)
    }
    fn get_player_field_unit_extra_effect_info(&self,
                                               notify_player_index: PlayerIndex,
                                               field_unit_extra_effect_info: FieldUnitExtraEffectInfo
    ) -> PlayerFieldUnitExtraEffectInfo {

        let mut player_field_unit_extra_effect_map = HashMap::new();
        player_field_unit_extra_effect_map.insert(notify_player_index, field_unit_extra_effect_info);

        PlayerFieldUnitExtraEffectInfo::new(player_field_unit_extra_effect_map)
    }
    fn get_player_field_unit_harmful_effect_info(&self,
                                               notify_player_index: PlayerIndex,
                                              field_unit_harmful_effect_info: FieldUnitHarmfulStatusInfo
    ) -> PlayerFieldUnitHarmfulEffectInfo {

        let mut player_field_unit_harmful_effect_map = HashMap::new();
        player_field_unit_harmful_effect_map.insert(notify_player_index, field_unit_harmful_effect_info);

        PlayerFieldUnitHarmfulEffectInfo::new(player_field_unit_harmful_effect_map)
    }
}

#[async_trait]
impl UiDataGeneratorRepository for UiDataGeneratorRepositoryImpl {
    async fn generate_use_my_hand_card_data(
        &mut self,
        used_hand_card_id: i32,
        used_hand_card_kind: KindsEnum
    ) -> (bool,
          PlayerHandCardUseInfo) {

        println!("UiDataGeneratorRepositoryImpl: generate_use_my_hand_card_data()");

        let player_hand_use_info_for_notice =
            self.get_player_hand_card_use_info(Opponent, used_hand_card_id, used_hand_card_kind);

        (true, player_hand_use_info_for_notice)
    }

    async fn generate_my_specific_unit_health_point_data(
        &mut self,
        my_unit_index: i32,
        my_unit_updated_health_point: i32
    ) -> (PlayerFieldUnitHealthPointInfo,
          PlayerFieldUnitHealthPointInfo) {

        println!("UiDataGeneratorRepositoryImpl: generate_my_specific_unit_health_point_data()");

        let field_unit_health_point_info =
            self.get_field_unit_health_info(my_unit_index, my_unit_updated_health_point);

        let player_field_unit_health_point_info_for_response =
            self.get_player_field_unit_health_point_info(You, field_unit_health_point_info.clone());
        let player_field_unit_health_point_info_for_notice =
            self.get_player_field_unit_health_point_info(Opponent, field_unit_health_point_info.clone());

        (player_field_unit_health_point_info_for_response,
         player_field_unit_health_point_info_for_notice)
    }
    async fn generate_my_main_character_health_point_data(
        &mut self,
        my_main_character_updated_health_point: i32
    ) -> (PlayerMainCharacterHealthPointInfo,
          PlayerMainCharacterHealthPointInfo) {

        println!("UiDataGeneratorRepositoryImpl: generate_my_main_character_health_point_data()");

        let player_main_character_health_point_info_for_response =
            self.get_player_main_character_health_point_info(You, my_main_character_updated_health_point.clone());
        let player_main_character_health_point_info_for_notice =
            self.get_player_main_character_health_point_info(Opponent, my_main_character_updated_health_point.clone());

        (player_main_character_health_point_info_for_response,
         player_main_character_health_point_info_for_notice)
    }

    async fn generate_opponent_main_character_health_point_data(
        &mut self,
        opponent_main_character_updated_health_point: i32
    ) -> (PlayerMainCharacterHealthPointInfo,
          PlayerMainCharacterHealthPointInfo) {

        println!("UiDataGeneratorRepositoryImpl: generate_opponent_main_character_health_point_data()");

        let player_main_character_health_point_info_for_response =
            self.get_player_main_character_health_point_info(Opponent, opponent_main_character_updated_health_point.clone());
        let player_main_character_health_point_info_for_notice =
            self.get_player_main_character_health_point_info(You, opponent_main_character_updated_health_point.clone());

        (player_main_character_health_point_info_for_response,
         player_main_character_health_point_info_for_notice)
    }

    async fn generate_my_specific_unit_energy_data(
        &mut self,
        unit_index: i32,
        updated_unit_energy_map: AttachedEnergyMap
    ) -> (PlayerFieldUnitEnergyInfo,
          PlayerFieldUnitEnergyInfo) {

        println!("UiDataGeneratorRepositoryImpl: generate_my_specific_unit_energy_data()");

        let field_unit_energy_info =
            self.get_field_unit_energy_info(unit_index, updated_unit_energy_map);

        let player_field_unit_energy_info_for_response =
            self.get_player_field_unit_energy_info(You, field_unit_energy_info.clone());
        let player_field_unit_energy_info_for_notice =
            self.get_player_field_unit_energy_info(Opponent, field_unit_energy_info.clone());

        (player_field_unit_energy_info_for_response,
         player_field_unit_energy_info_for_notice)
    }

    async fn generate_opponent_specific_unit_energy_data(
        &mut self,
        unit_index: i32,
        updated_unit_energy_map: AttachedEnergyMap
    ) -> (PlayerFieldUnitEnergyInfo,
          PlayerFieldUnitEnergyInfo) {

        println!("UiDataGeneratorRepositoryImpl: generate_opponent_specific_unit_energy_data()");

        let field_unit_energy_info =
            self.get_field_unit_energy_info(unit_index, updated_unit_energy_map);

        let player_field_unit_energy_info_for_response =
            self.get_player_field_unit_energy_info(Opponent, field_unit_energy_info.clone());
        let player_field_unit_energy_info_for_notice =
            self.get_player_field_unit_energy_info(You, field_unit_energy_info.clone());

        (player_field_unit_energy_info_for_response,
         player_field_unit_energy_info_for_notice)
    }

    async fn generate_use_my_field_energy_data(
        &mut self,
        remaining_field_energy: i32
    ) -> (PlayerFieldEnergyInfo,
          PlayerFieldEnergyInfo) {

        println!("UiDataGeneratorRepositoryImpl: generate_use_my_field_energy_data()");

        let player_field_energy_info_for_response =
            self.get_player_field_energy_info(You, remaining_field_energy);
        let player_field_energy_info_for_notice =
            self.get_player_field_energy_info(Opponent, remaining_field_energy);

        (player_field_energy_info_for_response,
         player_field_energy_info_for_notice)
    }

    async fn generate_use_my_deck_card_list_data(
        &mut self,
        deck_card_id_list: Vec<i32>,
    ) -> (PlayerDeckCardUseListInfo,
          PlayerDeckCardUseListInfo) {

        println!("UiDataGeneratorRepositoryImpl: generate_use_my_deck_card_list_data()");

        let player_deck_card_list_use_info_for_response =
            self.get_player_deck_card_list_use_info(You, deck_card_id_list.clone());
        let player_deck_card_list_use_info_for_notice =
            self.get_player_deck_card_list_use_info(Opponent, deck_card_id_list.clone());

        (player_deck_card_list_use_info_for_response,
         player_deck_card_list_use_info_for_notice)
    }

    async fn generate_draw_my_deck_data(
        &mut self,
        drawn_card_list: Vec<i32>
    ) -> (PlayerDrawnCardListInfo,
          PlayerDrawCountInfo) {

        println!("UiDataGeneratorRepositoryImpl: generate_draw_my_deck_data()");

        let player_drawn_card_list_info_for_response =
            self.get_player_drawn_card_list_info(You, drawn_card_list.clone());
        let player_draw_count_info_for_notice =
            self.get_player_draw_count_info(Opponent, drawn_card_list.clone());

        (player_drawn_card_list_info_for_response,
         player_draw_count_info_for_notice)
    }

    async fn generate_draw_opponent_deck_data(
        &mut self,
        drawn_card_list: Vec<i32>
    ) -> (PlayerDrawCountInfo,
          PlayerDrawnCardListInfo) {

        println!("UiDataGeneratorRepositoryImpl: generate_draw_opponent_deck_data()");

        let player_drawn_card_count_info_for_response =
            self.get_player_draw_count_info(Opponent, drawn_card_list.clone());
        let player_draw_count_list_for_notice =
            self.get_player_drawn_card_list_info(You, drawn_card_list.clone());

        (player_drawn_card_count_info_for_response,
         player_draw_count_list_for_notice)
    }

    async fn generate_search_my_deck_data(
        &mut self,
        found_card_list: Vec<i32>
    ) -> (PlayerSearchCardListInfo,
          PlayerSearchCountInfo) {

        println!("UiDataGeneratorRepositoryImpl: generate_search_my_deck_data()");

        let player_search_card_list_info_for_response =
            self.get_player_search_card_list_info(You, found_card_list.clone());
        let player_search_count_info_for_notice =
            self.get_player_search_count_info(Opponent, found_card_list.clone());

        (player_search_card_list_info_for_response,
         player_search_count_info_for_notice)
    }

    async fn generate_opponent_field_energy_data(
        &mut self,
        remaining_field_energy: i32
    ) -> (PlayerFieldEnergyInfo,
          PlayerFieldEnergyInfo) {

        println!("UiDataGeneratorRepositoryImpl: generate_opponent_field_energy_data()");

        let player_field_energy_info_for_response =
            self.get_player_field_energy_info(Opponent, remaining_field_energy);
        let player_field_energy_info_for_notice =
            self.get_player_field_energy_info(You, remaining_field_energy);

        (player_field_energy_info_for_response,
         player_field_energy_info_for_notice)
    }

    async fn generate_opponent_specific_unit_death_data(
        &mut self,
        dead_unit_index: i32
    ) -> (PlayerFieldUnitDeathInfo,
          PlayerFieldUnitDeathInfo) {

        println!("UiDataGeneratorRepositoryImpl: generate_opponent_specific_unit_death_data()");

        let field_unit_death_info =
            self.get_field_unit_death_info(dead_unit_index);

        let player_field_unit_death_info_for_response =
            self.get_player_field_unit_death_info(Opponent, field_unit_death_info.clone());
        let player_field_unit_death_info_for_notice =
            self.get_player_field_unit_death_info(You, field_unit_death_info.clone());

        (player_field_unit_death_info_for_response,
         player_field_unit_death_info_for_notice)
    }

    async fn generate_opponent_multiple_unit_death_data(
        &mut self,
        dead_unit_index_list: Vec<i32>
    ) -> (PlayerFieldUnitDeathInfo,
          PlayerFieldUnitDeathInfo) {

        println!("UiDataGeneratorRepositoryImpl: generate_opponent_multiple_unit_death_data()");

        let field_unit_death_info =
            self.get_field_unit_death_info_by_list(dead_unit_index_list);

        let player_field_unit_death_info_for_response =
            self.get_player_field_unit_death_info(Opponent, field_unit_death_info.clone());
        let player_field_unit_death_info_for_notice =
            self.get_player_field_unit_death_info(You, field_unit_death_info.clone());

        (player_field_unit_death_info_for_response,
         player_field_unit_death_info_for_notice)
    }
    async fn generate_my_multiple_unit_death_data(
        &mut self,
        dead_unit_index_list: Vec<i32>
    ) -> (PlayerFieldUnitDeathInfo,
          PlayerFieldUnitDeathInfo) {
        println!("UiDataGeneratorRepositoryImpl: generate_my_multiple_unit_death_data()");

        let field_unit_death_info =
            self.get_field_unit_death_info_by_list(dead_unit_index_list);

        let player_field_unit_death_info_for_response =
            self.get_player_field_unit_death_info(You, field_unit_death_info.clone());
        let player_field_unit_death_info_for_notice =
            self.get_player_field_unit_death_info(Opponent, field_unit_death_info.clone());

        (player_field_unit_death_info_for_response,
         player_field_unit_death_info_for_notice)
    }

    async fn generate_my_specific_unit_death_data(
        &mut self,
        dead_unit_index: i32
    ) -> (PlayerFieldUnitDeathInfo,
          PlayerFieldUnitDeathInfo) {

        println!("UiDataGeneratorRepositoryImpl: generate_my_specific_unit_death_data()");

        let field_unit_death_info =
            self.get_field_unit_death_info(dead_unit_index);

        let player_field_unit_death_info_for_response =
            self.get_player_field_unit_death_info(You, field_unit_death_info.clone());
        let player_field_unit_death_info_for_notice =
            self.get_player_field_unit_death_info(Opponent, field_unit_death_info.clone());

        (player_field_unit_death_info_for_response,
         player_field_unit_death_info_for_notice)
    }

    async fn generate_opponent_specific_unit_health_point_data(
        &mut self,
        opponent_unit_index: i32,
        opponent_unit_updated_health_point: i32
    ) -> (PlayerFieldUnitHealthPointInfo,
          PlayerFieldUnitHealthPointInfo) {

        println!("UiDataGeneratorRepositoryImpl: generate_opponent_specific_unit_health_point_data()");

        let field_unit_health_point_info =
            self.get_field_unit_health_info(opponent_unit_index, opponent_unit_updated_health_point);

        let player_field_unit_health_point_info_for_response =
            self.get_player_field_unit_health_point_info(Opponent, field_unit_health_point_info.clone());
        let player_field_unit_health_point_info_for_notice =
            self.get_player_field_unit_health_point_info(You, field_unit_health_point_info.clone());

        (player_field_unit_health_point_info_for_response,
         player_field_unit_health_point_info_for_notice)
    }

    async fn generate_opponent_multiple_unit_health_point_data(
        &mut self,
        opponent_unit_health_point_tuple_list: Vec<(i32, i32)>
    ) -> (PlayerFieldUnitHealthPointInfo,
          PlayerFieldUnitHealthPointInfo) {

        println!("UiDataGeneratorRepositoryImpl: generate_opponent_multiple_unit_health_point_data()");

        let field_unit_health_point_info =
            self.get_field_unit_health_info_from_tuple_list(opponent_unit_health_point_tuple_list);

        let player_field_unit_health_point_info_for_response =
            self.get_player_field_unit_health_point_info(Opponent, field_unit_health_point_info.clone());
        let player_field_unit_health_point_info_for_notice =
            self.get_player_field_unit_health_point_info(You, field_unit_health_point_info.clone());

        (player_field_unit_health_point_info_for_response,
         player_field_unit_health_point_info_for_notice)
    }

    async fn generate_my_multiple_unit_health_point_data(
        &mut self,
        my_unit_health_point_tuple_list: Vec<(i32, i32)>
    ) -> (PlayerFieldUnitHealthPointInfo,
          PlayerFieldUnitHealthPointInfo) {

        println!("UiDataGeneratorRepositoryImpl: generate_my_multiple_unit_health_point_data()");

        let field_unit_health_point_info =
            self.get_field_unit_health_info_from_tuple_list(my_unit_health_point_tuple_list);

        let player_field_unit_health_point_info_for_response =
            self.get_player_field_unit_health_point_info(You, field_unit_health_point_info.clone());
        let player_field_unit_health_point_info_for_notice =
            self.get_player_field_unit_health_point_info(Opponent, field_unit_health_point_info.clone());

        (player_field_unit_health_point_info_for_response,
         player_field_unit_health_point_info_for_notice)
    }

    async fn generate_my_main_character_survival_data(
        &mut self,
        my_main_character_status: StatusMainCharacterEnum
    ) -> (PlayerMainCharacterSurvivalInfo,
          PlayerMainCharacterSurvivalInfo) {
        println!("UiDataGeneratorRepositoryImpl: generate_my_main_character_survival_data()");


        let player_main_character_survival_info_for_response =
            self.get_player_main_character_survival_info(You, my_main_character_status.clone());
        let player_main_character_survival_info_for_notice =
            self.get_player_main_character_survival_info(Opponent, my_main_character_status.clone());

        (player_main_character_survival_info_for_response,
         player_main_character_survival_info_for_notice)
    }

    async fn generate_opponent_main_character_survival_data(
        &mut self,
        my_main_character_status: StatusMainCharacterEnum
    ) -> (PlayerMainCharacterSurvivalInfo,
          PlayerMainCharacterSurvivalInfo) {

        println!("UiDataGeneratorRepositoryImpl: generate_opponent_main_character_survival_data()");

        let player_main_character_survival_info_for_response =
            self.get_player_main_character_survival_info(Opponent, my_main_character_status.clone());
        let player_main_character_survival_info_for_notice =
            self.get_player_main_character_survival_info(You, my_main_character_status.clone());

        (player_main_character_survival_info_for_response,
         player_main_character_survival_info_for_notice)
    }

    async fn generate_my_specific_unit_extra_effect_data(
        &mut self,
        my_extra_effect_unit_index:i32,
        my_unit_extra_effect_list:Vec<ExtraEffect>
    ) -> (PlayerFieldUnitExtraEffectInfo,
          PlayerFieldUnitExtraEffectInfo) {

        println!("UiDataGeneratorRepositoryImpl: generate_my_specific_unit_extra_effect_data()");

        let field_unit_extra_effect_info =
            self.get_field_unit_extra_effect_info(my_extra_effect_unit_index, my_unit_extra_effect_list);

        let player_unit_extra_effect_info_for_response =
            self.get_player_field_unit_extra_effect_info(You, field_unit_extra_effect_info.clone());
        let player_unit_extra_effect_info_for_notice =
            self.get_player_field_unit_extra_effect_info(Opponent, field_unit_extra_effect_info.clone());

        (player_unit_extra_effect_info_for_response,
         player_unit_extra_effect_info_for_notice)
    }
    async fn generate_opponent_specific_unit_extra_effect_data(
        &mut self,
        opponent_extra_effect_unit_index:i32,
        opponent_unit_extra_effect_list:Vec<ExtraEffect>
    ) -> (PlayerFieldUnitExtraEffectInfo,
          PlayerFieldUnitExtraEffectInfo) {
        println!("UiDataGeneratorRepositoryImpl: generate_opponent_specific_unit_extra_effect_data()");

        let field_unit_extra_effect_info =
            self.get_field_unit_extra_effect_info(opponent_extra_effect_unit_index, opponent_unit_extra_effect_list);

        let player_unit_extra_effect_info_for_response =
            self.get_player_field_unit_extra_effect_info(Opponent, field_unit_extra_effect_info.clone());
        let player_unit_extra_effect_info_for_notice =
            self.get_player_field_unit_extra_effect_info(You, field_unit_extra_effect_info.clone());

        (player_unit_extra_effect_info_for_response,
         player_unit_extra_effect_info_for_notice)
    }
    async fn generate_my_specific_unit_harmful_effect_data(
        &mut self,
        my_harmful_effect_unit_index:i32,
        my_unit_harmful_effect_list:Vec<ExtraEffect>
    ) -> (PlayerFieldUnitHarmfulEffectInfo,
          PlayerFieldUnitHarmfulEffectInfo) {
        println!("UiDataGeneratorRepositoryImpl: generate_my_specific_unit_harmful_effect_data()");

        let field_unit_harmful_effect_info =
            self.get_field_unit_harmful_effect_info(my_harmful_effect_unit_index, my_unit_harmful_effect_list);

        let player_unit_harmful_effect_info_for_response =
            self.get_player_field_unit_harmful_effect_info(You, field_unit_harmful_effect_info.clone());
        let player_unit_harmful_effect_info_for_notice =
            self.get_player_field_unit_harmful_effect_info(Opponent, field_unit_harmful_effect_info.clone());

        (player_unit_harmful_effect_info_for_response,
         player_unit_harmful_effect_info_for_notice)
    }
    async fn generate_opponent_specific_unit_harmful_effect_data(
        &mut self,
        opponent_harmful_effect_unit_index:i32,
        opponent_unit_harmful_effect_list:Vec<ExtraEffect>
    ) -> (PlayerFieldUnitHarmfulEffectInfo,
          PlayerFieldUnitHarmfulEffectInfo) {
        println!("UiDataGeneratorRepositoryImpl: generate_my_specific_unit_harmful_effect_data()");

        let field_unit_harmful_effect_info =
            self.get_field_unit_harmful_effect_info(opponent_harmful_effect_unit_index, opponent_unit_harmful_effect_list);

        let player_unit_harmful_effect_info_for_response =
            self.get_player_field_unit_harmful_effect_info(Opponent, field_unit_harmful_effect_info.clone());
        let player_unit_harmful_effect_info_for_notice =
            self.get_player_field_unit_harmful_effect_info(You, field_unit_harmful_effect_info.clone());

        (player_unit_harmful_effect_info_for_response,
         player_unit_harmful_effect_info_for_notice)
    }

    async fn generate_my_multiple_unit_extra_effect_data(
        &mut self,
        my_unit_extra_effect_tuple_list: Vec<(i32, Vec<ExtraEffect>)>
    ) -> (PlayerFieldUnitExtraEffectInfo,
          PlayerFieldUnitExtraEffectInfo) {

        println!("UiDataGeneratorRepositoryImpl: generate_my_multiple_unit_extra_effect_data()");

        let field_unit_extra_effect_info =
            self.get_field_unit_extra_effect_from_tuple_list(my_unit_extra_effect_tuple_list);

        let player_field_unit_extra_effect_info_for_response =
            self.get_player_field_unit_extra_effect_info(You, field_unit_extra_effect_info.clone());
        let player_field_unit_extra_effect_info_for_notice =
            self.get_player_field_unit_extra_effect_info(Opponent, field_unit_extra_effect_info.clone());

        (player_field_unit_extra_effect_info_for_response,
         player_field_unit_extra_effect_info_for_notice)
    }
    async fn generate_opponent_multiple_unit_extra_effect_data(
        &mut self,
        opponent_unit_extra_effect_tuple_list: Vec<(i32, Vec<ExtraEffect>)>
    ) -> (PlayerFieldUnitExtraEffectInfo,
          PlayerFieldUnitExtraEffectInfo) {
        println!("UiDataGeneratorRepositoryImpl: generate_opponent_multiple_unit_extra_effect_data()");

        let field_unit_extra_effect_info =
            self.get_field_unit_extra_effect_from_tuple_list(opponent_unit_extra_effect_tuple_list);

        let player_field_unit_extra_effect_info_for_response =
            self.get_player_field_unit_extra_effect_info(Opponent, field_unit_extra_effect_info.clone());
        let player_field_unit_extra_effect_info_for_notice =
            self.get_player_field_unit_extra_effect_info(You, field_unit_extra_effect_info.clone());

        (player_field_unit_extra_effect_info_for_response,
         player_field_unit_extra_effect_info_for_notice)
    }
    async fn generate_my_multiple_unit_harmful_effect_data(
        &mut self,
        my_unit_harmful_effect_tuple_list: Vec<(i32, Vec<ExtraEffect>)>
    ) -> (PlayerFieldUnitHarmfulEffectInfo,
          PlayerFieldUnitHarmfulEffectInfo){
        println!("UiDataGeneratorRepositoryImpl: generate_my_multiple_unit_harmful_effect_data()");

        let field_unit_harmful_effect_info =
            self.get_field_unit_harmful_effect_from_tuple_list(my_unit_harmful_effect_tuple_list);

        let player_field_unit_harmful_effect_info_for_response =
            self.get_player_field_unit_harmful_effect_info(You, field_unit_harmful_effect_info.clone());
        let player_field_unit_harmful_effect_info_for_notice =
            self.get_player_field_unit_harmful_effect_info(Opponent, field_unit_harmful_effect_info.clone());

        (player_field_unit_harmful_effect_info_for_response,
         player_field_unit_harmful_effect_info_for_notice)

    }
    async fn generate_opponent_multiple_unit_harmful_effect_data(
        &mut self,
        opponent_unit_harmful_effect_tuple_list: Vec<(i32, Vec<ExtraEffect>)>
    ) -> (PlayerFieldUnitHarmfulEffectInfo,
          PlayerFieldUnitHarmfulEffectInfo) {
        println!("UiDataGeneratorRepositoryImpl: generate_opponent_multiple_unit_harmful_effect_data()");

        let field_unit_harmful_effect_info =
            self.get_field_unit_harmful_effect_from_tuple_list(opponent_unit_harmful_effect_tuple_list);

        let player_field_unit_harmful_effect_info_for_response =
            self.get_player_field_unit_harmful_effect_info(Opponent, field_unit_harmful_effect_info.clone());
        let player_field_unit_harmful_effect_info_for_notice =
            self.get_player_field_unit_harmful_effect_info(You, field_unit_harmful_effect_info.clone());

        (player_field_unit_harmful_effect_info_for_response,
         player_field_unit_harmful_effect_info_for_notice)
    }

    async fn generate_opponent_deck_card_lost_data(
        &mut self,
        opponent_lost_deck_card_list: Vec<i32>
    ) -> (PlayerDeckCardLostListInfo,
          PlayerDeckCardLostListInfo) {

        println!("UiDataGeneratorRepositoryImpl: generate_my_multiple_unit_extra_effect_data()");

        let player_deck_card_lost_list_info_for_response =
            self.get_player_deck_card_lost_list_info(Opponent, opponent_lost_deck_card_list.clone());
        let player_deck_card_lost_list_info_for_notice =
            self.get_player_deck_card_lost_list_info(You, opponent_lost_deck_card_list.clone());

        (player_deck_card_lost_list_info_for_response,
         player_deck_card_lost_list_info_for_notice)
    }

    async fn generate_my_specific_unit_basic_attack_data(
        &mut self,
        attacker_unit_index: i32,
        target_unit_index: i32,
    ) -> (PlayerFieldUnitAttackInfo,
          PlayerFieldUnitAttackInfo) {

        println!("UiDataGeneratorRepositoryImpl: generate_my_specific_unit_basic_attack_data()");

        let field_unit_basic_attack_info_for_response =
            self.get_field_unit_basic_attack_info(attacker_unit_index, Opponent, target_unit_index);
        let field_unit_basic_attack_info_for_notice =
            self.get_field_unit_basic_attack_info(attacker_unit_index, You, target_unit_index);

        let player_field_unit_attack_info_for_response =
            self.get_player_field_unit_attack_info(You, field_unit_basic_attack_info_for_response);
        let player_field_unit_attack_info_for_notice =
            self.get_player_field_unit_attack_info(Opponent, field_unit_basic_attack_info_for_notice);

        (player_field_unit_attack_info_for_response,
         player_field_unit_attack_info_for_notice)
    }

    async fn generate_my_specific_unit_active_skill_use_data(
        &mut self,
        attacker_unit_index: i32,
        target_unit_index: i32,
        active_skill_index: i32,
    ) -> (PlayerFieldUnitAttackInfo,
          PlayerFieldUnitAttackInfo) {

        println!("UiDataGeneratorRepositoryImpl: generate_my_specific_unit_active_skill_use_data()");

        let field_unit_active_skill_attack_info_for_response =
            self.get_field_unit_active_skill_attack_info(attacker_unit_index, Opponent, target_unit_index, active_skill_index);
        let field_unit_active_skill_attack_info_for_notice =
            self.get_field_unit_active_skill_attack_info(attacker_unit_index, You, target_unit_index, active_skill_index);

        let player_field_unit_attack_info_for_response =
            self.get_player_field_unit_attack_info(You, field_unit_active_skill_attack_info_for_response);
        let player_field_unit_attack_info_for_notice =
            self.get_player_field_unit_attack_info(Opponent, field_unit_active_skill_attack_info_for_notice);

        (player_field_unit_attack_info_for_response,
         player_field_unit_attack_info_for_notice)
    }
    async fn generate_my_specific_unit_passive_skill_use_data(
        &mut self,
        attacker_unit_index: i32,
        target_unit_index: i32,
        passive_skill_index: i32,
    ) -> (PlayerFieldUnitAttackInfo,
          PlayerFieldUnitAttackInfo) {

        println!("UiDataGeneratorRepositoryImpl: generate_my_specific_unit_active_skill_use_data()");

        let field_unit_passive_skill_attack_info_for_response =
            self.get_field_unit_passive_attack_to_unit_info(attacker_unit_index, Opponent, target_unit_index, passive_skill_index);
        let field_unit_passive_skill_attack_info_for_notice =
            self.get_field_unit_passive_attack_to_unit_info(attacker_unit_index, You, target_unit_index, passive_skill_index);

        let player_field_unit_attack_info_for_response =
            self.get_player_field_unit_attack_info(You, field_unit_passive_skill_attack_info_for_response);
        let player_field_unit_attack_info_for_notice =
            self.get_player_field_unit_attack_info(Opponent, field_unit_passive_skill_attack_info_for_notice);

        (player_field_unit_attack_info_for_response,
         player_field_unit_attack_info_for_notice)
    }

}