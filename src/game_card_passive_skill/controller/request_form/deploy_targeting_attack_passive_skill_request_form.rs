use std::collections::HashMap;
use crate::battle_room::service::request::find_opponent_by_account_id_request::FindOpponentByAccountIdRequest;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_active_skill::service::request::summary_active_skill_effect_request::SummaryActiveSkillEffectRequest;
use crate::game_card_passive_skill::entity::passive_skill_casting_condition::PassiveSkillCastingCondition;
use crate::game_card_passive_skill::entity::passive_skill_type::PassiveSkillType;
use crate::game_card_passive_skill::service::request::summary_deploy_passive_skill_effect_request::SummaryDeployPassiveSkillEffectRequest;
use crate::game_card_passive_skill::service::request::summary_passive_skill_effect_by_index_request::SummaryPassiveSkillEffectByIndexRequest;
use crate::game_card_passive_skill::service::request::summary_passive_skill_effect_request::SummaryPassiveSkillEffectRequest;
use crate::game_card_passive_skill::service::response::summary_passive_skill_effect_by_index_response::SummaryPassiveSkillEffectByIndexResponse;
use crate::game_field_unit::entity::extra_effect::ExtraEffect;
use crate::game_field_unit::entity::extra_status_effect::ExtraStatusEffect;
use crate::game_field_unit::service::request::acquire_unit_extra_effect_request::AcquireUnitExtraEffectRequest;
use crate::game_field_unit::service::request::acquire_unit_harmful_status_effect_request::AcquireUnitHarmfulStatusEffectRequest;
use crate::game_field_unit::service::request::apply_damage_to_target_unit_index_request::ApplyDamageToTargetUnitIndexRequest;
use crate::game_field_unit::service::request::attack_target_unit_with_extra_effect_request::AttackTargetUnitWithExtraEffectRequest;
use crate::game_field_unit::service::request::execute_index_passive_of_unit_request::ExecuteIndexPassiveOfUnitRequest;
use crate::game_field_unit::service::request::execute_turn_action_request::ExecuteTurnActionRequest;
use crate::game_field_unit::service::request::find_active_skill_usage_unit_id_by_index_request::FindActiveSkillUsageUnitIdByIndexRequest;
use crate::game_field_unit::service::request::find_target_unit_id_by_index_request::FindTargetUnitIdByIndexRequest;
use crate::game_field_unit::service::request::get_current_health_point_of_field_unit_by_index_request::GetCurrentHealthPointOfFieldUnitByIndexRequest;
use crate::game_field_unit::service::request::get_passive_skill_usable_request::GetPassiveSkillUsableRequest;
use crate::game_field_unit::service::request::judge_death_of_unit_request::JudgeDeathOfUnitRequest;
use crate::game_field_unit_action_possibility_validator::service::request::is_using_active_skill_possible_request::IsUsingActiveSkillPossibleRequest;
use crate::game_field_unit_action_possibility_validator::service::request::is_using_deploy_passive_skill_possible_request::IsUsingDeployPassiveSkillPossibleRequest;
use crate::game_protocol_validation::service::request::is_this_your_turn_request::IsThisYourTurnRequest;
use crate::game_tomb::service::request::place_to_tomb_request::PlaceToTombRequest;
use crate::notify_player_action_info::service::request::notice_deploy_targeting_attack_passive_skill_to_unit_request::NoticeDeployTargetingAttackPassiveSkillToUnitRequest;
use crate::notify_player_action_info::service::request::notice_targeting_attack_active_skill_to_unit_request::NoticeTargetingAttackActiveSkillToUnitRequest;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;
use crate::ui_data_generator::entity::field_unit_basic_attack_info::FieldUnitAttackInfo;
use crate::ui_data_generator::entity::field_unit_death_info::FieldUnitDeathInfo;
use crate::ui_data_generator::entity::field_unit_harmful_status_info::FieldUnitHarmfulStatusInfo;
use crate::ui_data_generator::entity::field_unit_health_point_info::FieldUnitHealthPointInfo;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::ui_data_generator::service::request::generate_my_specific_unit_passive_skill_use_data_request::GenerateMySpecificUnitPassiveSkillUseDataRequest;
use crate::ui_data_generator::service::request::generate_opponent_specific_unit_death_data_request::GenerateOpponentSpecificUnitDeathDataRequest;
use crate::ui_data_generator::service::request::generate_opponent_specific_unit_harmful_effect_data_request::GenerateOpponentSpecificUnitHarmfulEffectDataRequest;
use crate::ui_data_generator::service::request::generate_opponent_specific_unit_health_point_data_request::GenerateOpponentSpecificUnitHealthPointDataRequest;

pub struct DeployTargetingAttackPassiveSkillRequestForm {
    session_id: String,
    unit_card_index: String,
    opponent_target_card_index: String,
    usage_skill_index: String,
}

impl DeployTargetingAttackPassiveSkillRequestForm {
    pub fn new(session_id: String,
               unit_card_index: String,
               opponent_target_card_index: String,
               usage_skill_index: String) -> Self {

        DeployTargetingAttackPassiveSkillRequestForm {
            session_id: session_id.to_string(),
            unit_card_index: unit_card_index.to_string(),
            opponent_target_card_index: opponent_target_card_index.to_string(),
            usage_skill_index: usage_skill_index.to_string(),
        }
    }

    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }

    pub fn get_unit_card_index(&self) -> &str {
        &self.unit_card_index
    }

    pub fn get_opponent_target_card_index(&self) -> &str {
        &self.opponent_target_card_index
    }

    pub fn get_usage_skill_index(&self) -> &str {
        &self.usage_skill_index
    }

    pub fn to_session_validation_request(&self) -> GetValueWithKeyRequest {
        GetValueWithKeyRequest::new(self.session_id.clone().as_str())
    }

    pub fn to_is_this_your_turn_request(&self,
                                        account_unique_id: i32) -> IsThisYourTurnRequest {
        IsThisYourTurnRequest::new(
            account_unique_id)
    }

    pub fn to_execute_index_passive_of_unit_request(&self,
                                          account_unique_id: i32,
                                          unit_card_index: i32,
                                          passive_skill_index: i32) -> ExecuteIndexPassiveOfUnitRequest {
        ExecuteIndexPassiveOfUnitRequest::new(
            account_unique_id,
            unit_card_index,
            passive_skill_index)
    }

    pub fn to_summary_passive_skill_effect_by_index_request(&self, unit_card_id: i32, usage_skill_index: i32) -> SummaryPassiveSkillEffectByIndexRequest {
        SummaryPassiveSkillEffectByIndexRequest::new(
            unit_card_id,
            usage_skill_index)
    }
    pub fn to_summary_deploy_passive_skill_effect_request(&self, unit_card_id: i32) -> SummaryDeployPassiveSkillEffectRequest {
        SummaryDeployPassiveSkillEffectRequest::new(unit_card_id)
    }
    pub fn to_acquire_unit_extra_effect_request(&self,
                                           account_unique_id: i32,
                                           unit_index: i32) -> AcquireUnitExtraEffectRequest {
        AcquireUnitExtraEffectRequest::new(
            account_unique_id,
            unit_index)
    }

    pub fn to_find_opponent_by_account_id_request(&self, account_unique_id: i32) -> FindOpponentByAccountIdRequest {
        FindOpponentByAccountIdRequest::new(
            account_unique_id)
    }

    pub fn to_attack_target_with_extra_effect_request(&self,
                                                      opponent_unique_id: i32,
                                                      damage: i32,
                                                      extra_status_effect_list: Vec<ExtraStatusEffect>,
                                                      target_unit_index: i32) -> AttackTargetUnitWithExtraEffectRequest {
        AttackTargetUnitWithExtraEffectRequest::new(
            opponent_unique_id,
            damage,
            extra_status_effect_list,
            target_unit_index)
    }

    pub fn to_apply_damage_to_target_unit_index_request(&self, opponent_unique_id: i32, opponent_target_unit_index: i32, damage: i32) -> ApplyDamageToTargetUnitIndexRequest {
        ApplyDamageToTargetUnitIndexRequest::new(
            opponent_unique_id, opponent_target_unit_index, damage)
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
    pub fn to_find_unit_id_by_index_request(&self,
                                            account_unique_id: i32,
                                            unit_index: i32) -> FindTargetUnitIdByIndexRequest {
        FindTargetUnitIdByIndexRequest::new(
            account_unique_id, unit_index)
    }
    pub fn to_is_using_deploy_passive_skill_possible_request(&self,
                                                        account_unique_id: i32,
                                                        unit_index: i32,
                                                        usage_skill_index: i32,
                                                        passive_skill_casting_condition: Vec<PassiveSkillCastingCondition>) -> IsUsingDeployPassiveSkillPossibleRequest {
        IsUsingDeployPassiveSkillPossibleRequest::new(
            account_unique_id,
            unit_index,
            usage_skill_index,
            passive_skill_casting_condition
        )
    }
    pub fn to_get_passive_skill_usable_list_request(&self, account_unique_id: i32, unit_index: i32) -> GetPassiveSkillUsableRequest {
        GetPassiveSkillUsableRequest::new(
            account_unique_id,
            unit_index
        )
    }
    pub fn to_place_dead_unit_to_tomb_request(
        &self,
        account_unique_id: i32,
        dead_unit_card_id: i32) -> PlaceToTombRequest {

        PlaceToTombRequest::new(
            account_unique_id, dead_unit_card_id)
    }

    pub fn to_get_current_health_point_of_field_unit_by_index_request(
        &self,
        account_unique_id: i32,
        unit_index: i32
    ) -> GetCurrentHealthPointOfFieldUnitByIndexRequest {

        GetCurrentHealthPointOfFieldUnitByIndexRequest::new(
            account_unique_id, unit_index)
    }
    pub fn to_acquire_unit_harmful_status_effect_request(
        &self,
        opponent_unique_id: i32,
        opponent_unit_index: i32) -> AcquireUnitHarmfulStatusEffectRequest {

        AcquireUnitHarmfulStatusEffectRequest::new(
            opponent_unique_id, opponent_unit_index)
    }
    pub fn to_generate_my_specific_unit_passive_skill_use_data_request(
        &self,
        attacker_unit_index: i32,
        target_unit_index: i32,
        passive_skill_index: i32) -> GenerateMySpecificUnitPassiveSkillUseDataRequest {

        GenerateMySpecificUnitPassiveSkillUseDataRequest::new(
            attacker_unit_index, target_unit_index, passive_skill_index)
    }
    pub fn to_generate_opponent_specific_unit_health_point_data_request(
        &self,
        opponent_unit_index: i32,
        opponent_unit_updated_health_point: i32
    ) -> GenerateOpponentSpecificUnitHealthPointDataRequest {

        GenerateOpponentSpecificUnitHealthPointDataRequest::new(
            opponent_unit_index, opponent_unit_updated_health_point)
    }

    pub fn to_generate_opponent_specific_unit_harmful_effect_data_request(
        &self,
        opponent_unit_index: i32,
        opponent_unit_harmful_status_list: Vec<ExtraEffect>,
    ) -> GenerateOpponentSpecificUnitHarmfulEffectDataRequest {

        GenerateOpponentSpecificUnitHarmfulEffectDataRequest::new(
            opponent_unit_index, opponent_unit_harmful_status_list)
    }

    pub fn to_generate_opponent_specific_unit_death_data_request(
        &self,
        opponent_dead_unit_index: i32
    ) -> GenerateOpponentSpecificUnitDeathDataRequest {

        GenerateOpponentSpecificUnitDeathDataRequest::new(
            opponent_dead_unit_index)
    }
    pub fn to_notice_deploy_targeting_attack_passive_skill_to_unit_request(
        &self,
        opponent_unique_id: i32,
        player_field_unit_attack_map_for_notice: HashMap<PlayerIndex, FieldUnitAttackInfo>,
        player_field_unit_health_point_map_for_notice: HashMap<PlayerIndex, FieldUnitHealthPointInfo>,
        player_field_unit_harmful_effect_map_for_notice: HashMap<PlayerIndex, FieldUnitHarmfulStatusInfo>,
        player_field_unit_death_map_for_notice: HashMap<PlayerIndex, FieldUnitDeathInfo>
    ) -> NoticeDeployTargetingAttackPassiveSkillToUnitRequest {

        NoticeDeployTargetingAttackPassiveSkillToUnitRequest::new(
            opponent_unique_id,
            player_field_unit_attack_map_for_notice,
            player_field_unit_health_point_map_for_notice,
            player_field_unit_harmful_effect_map_for_notice,
            player_field_unit_death_map_for_notice)
    }
}
