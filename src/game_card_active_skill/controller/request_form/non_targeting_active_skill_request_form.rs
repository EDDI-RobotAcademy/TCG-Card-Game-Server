use std::collections::HashMap;
use crate::battle_room::service::request::find_opponent_by_account_id_request::FindOpponentByAccountIdRequest;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_active_skill::service::request::summary_active_skill_effect_request::SummaryActiveSkillEffectRequest;
use crate::game_field_unit::service::request::acquire_unit_extra_effect_request::AcquireUnitExtraEffectRequest;
use crate::game_field_unit::service::request::apply_catastrophic_damage_to_field_unit_request::ApplyCatastrophicDamageToFieldUnitRequest;
use crate::game_field_unit::service::request::apply_damage_to_target_unit_index_request::ApplyDamageToTargetUnitIndexRequest;
use crate::game_field_unit::service::request::execute_turn_action_request::ExecuteTurnActionRequest;
use crate::game_field_unit::service::request::find_active_skill_usage_unit_id_by_index_request::FindActiveSkillUsageUnitIdByIndexRequest;
use crate::game_field_unit::service::request::find_target_unit_id_by_index_request::FindTargetUnitIdByIndexRequest;
use crate::game_field_unit::service::request::judge_death_of_unit_request::JudgeDeathOfUnitRequest;
use crate::game_field_unit_action_possibility_validator::service::request::is_using_active_skill_possible_request::IsUsingActiveSkillPossibleRequest;
use crate::game_tomb::service::request::place_to_tomb_request::PlaceToTombRequest;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;

pub struct NonTargetingActiveSkillRequestForm {
    session_id: String,
    unit_card_index: String,
    usage_skill_index: String,
}

impl NonTargetingActiveSkillRequestForm {
    pub fn new(session_id: String,
               unit_card_index: String,
               usage_skill_index: String) -> Self {

        NonTargetingActiveSkillRequestForm {
            session_id: session_id.to_string(),
            unit_card_index: unit_card_index.to_string(),
            usage_skill_index: usage_skill_index.to_string(),
        }
    }

    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }

    pub fn get_unit_card_index(&self) -> &str {
        &self.unit_card_index
    }

    pub fn get_usage_skill_index(&self) -> &str {
        &self.usage_skill_index
    }

    pub fn to_session_validation_request(&self) -> GetValueWithKeyRequest {
        GetValueWithKeyRequest::new(
            self.session_id.clone().as_str())
    }

    pub fn to_execute_turn_action_request(&self,
                                          account_unique_id: i32,
                                          attacker_unit_card_index: i32) -> ExecuteTurnActionRequest {
        ExecuteTurnActionRequest::new(
            account_unique_id, attacker_unit_card_index)
    }

    pub fn to_find_active_skill_usage_unit_id_by_index_request(&self,
                                                               account_unique_id: i32,
                                                               unit_card_index: i32) -> FindActiveSkillUsageUnitIdByIndexRequest {
        FindActiveSkillUsageUnitIdByIndexRequest::new(
            account_unique_id,
            unit_card_index)
    }

    pub fn to_summary_active_skill_effect_request(&self,
                                                  unit_card_index: i32,
                                                  usage_skill_index: i32) -> SummaryActiveSkillEffectRequest {
        SummaryActiveSkillEffectRequest::new(
            unit_card_index,
            usage_skill_index)
    }

    pub fn to_find_target_unit_id_by_index_request(&self,
                                                   account_unique_id: i32,
                                                   unit_card_index: i32) -> FindTargetUnitIdByIndexRequest {
        FindTargetUnitIdByIndexRequest::new(
            account_unique_id,
            unit_card_index)
    }

    pub fn to_is_using_active_skill_possible_request(&self,
                                                     account_unique_id: i32,
                                                     field_unit_index: i32,
                                                     skill_required_energy_map: HashMap<RaceEnum, i32>) -> IsUsingActiveSkillPossibleRequest {
        IsUsingActiveSkillPossibleRequest::new(
            account_unique_id,
            field_unit_index,
            skill_required_energy_map)
    }

    pub fn to_find_opponent_by_account_id_request(&self,
                                                  account_unique_id: i32) -> FindOpponentByAccountIdRequest {
        FindOpponentByAccountIdRequest::new(
            account_unique_id)
    }

    pub fn to_acquire_unit_extra_effect_request(&self,
                                                account_unique_id: i32,
                                                unit_index: i32) -> AcquireUnitExtraEffectRequest {
        AcquireUnitExtraEffectRequest::new(
            account_unique_id,
            unit_index)
    }

    pub fn to_apply_damage_to_target_unit_index_request(&self, opponent_unique_id: i32, opponent_target_unit_index: i32, damage: i32) -> ApplyDamageToTargetUnitIndexRequest {
        ApplyDamageToTargetUnitIndexRequest::new(
            opponent_unique_id,
            opponent_target_unit_index,
            damage)
    }

    pub fn to_apply_catastrophic_damage_to_field_unit_request(&self,
                                                              opponent_unique_id: i32,
                                                              damage: i32) -> ApplyCatastrophicDamageToFieldUnitRequest {
        ApplyCatastrophicDamageToFieldUnitRequest::new(
            opponent_unique_id,
            damage)
    }

    pub fn to_judge_death_of_unit_request(&self,
                                          account_unique_id: i32,
                                          unit_index: i32) -> JudgeDeathOfUnitRequest {
        JudgeDeathOfUnitRequest::new(
            account_unique_id,
            unit_index)
    }

    pub fn to_add_dead_unit_to_tomb_request(&self,
                                            account_unique_id: i32,
                                            dead_card_id: i32) -> PlaceToTombRequest {
        PlaceToTombRequest::new(
            account_unique_id,
            dead_card_id)
    }
}