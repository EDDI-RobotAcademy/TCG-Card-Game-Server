use crate::battle_room::service::request::find_opponent_by_account_id_request::FindOpponentByAccountIdRequest;
use crate::game_card_passive_skill::service::request::summary_passive_skill_effect_request::SummaryPassiveSkillEffectRequest;
use crate::game_field_unit::entity::extra_status_effect::ExtraStatusEffect;
use crate::game_field_unit::service::request::acquire_unit_attack_point_request::AcquireUnitAttackPointRequest;
use crate::game_field_unit::service::request::acquire_unit_extra_effect_request::AcquireUnitExtraEffectRequest;
use crate::game_field_unit::service::request::attack_target_unit_with_extra_effect_request::AttackTargetUnitWithExtraEffectRequest;
use crate::game_field_unit::service::request::check_turn_action_request::CheckTurnActionRequest;
use crate::game_field_unit::service::request::execute_turn_action_request::ExecuteTurnActionRequest;
use crate::game_field_unit::service::request::find_target_unit_id_by_index_request::FindTargetUnitIdByIndexRequest;
use crate::game_field_unit::service::request::judge_death_of_unit_request::JudgeDeathOfUnitRequest;
use crate::game_tomb::service::request::place_to_tomb_request::PlaceToTombRequest;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;

pub struct AttackUnitRequestForm {
    session_id: String,
    attacker_unit_index: String,
    target_unit_index: String,
}

impl AttackUnitRequestForm {
    pub fn new(session_id: String, attacker_unit_index: String, target_unit_index: String) -> Self {
        AttackUnitRequestForm {
            session_id: session_id.to_string(),
            attacker_unit_index: attacker_unit_index.to_string(),
            target_unit_index: target_unit_index.to_string(),
        }
    }

    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }

    pub fn get_attacker_unit_index(&self) -> &str {
        &self.attacker_unit_index
    }

    pub fn get_target_unit_index(&self) -> &str {
        &self.target_unit_index
    }

    pub fn to_session_validation_request(&self) -> GetValueWithKeyRequest {
        GetValueWithKeyRequest::new(self.session_id.clone().as_str())
    }

    pub fn to_check_turn_action_request(&self,
                                        account_unique_id: i32,
                                        attacker_unit_card_index: i32) -> CheckTurnActionRequest {
        CheckTurnActionRequest::new(
            account_unique_id, attacker_unit_card_index)
    }

    pub fn to_execute_turn_action_request(&self,
                                          account_unique_id: i32,
                                          attacker_unit_card_index: i32) -> ExecuteTurnActionRequest {
        ExecuteTurnActionRequest::new(
            account_unique_id, attacker_unit_card_index)
    }

    pub fn to_acquire_unit_attack_point_request(&self,
                                                account_unique_id: i32,
                                                attacker_unit_card_index: i32) -> AcquireUnitAttackPointRequest {
        AcquireUnitAttackPointRequest::new(
            account_unique_id, attacker_unit_card_index)
    }

    pub fn to_acquire_unit_extra_effect_request(&self,
                                                account_unique_id: i32,
                                                attacker_unit_card_index: i32) -> AcquireUnitExtraEffectRequest {
        AcquireUnitExtraEffectRequest::new(
            account_unique_id, attacker_unit_card_index)
    }

    pub fn to_find_unit_id_by_index_request(&self,
                                            account_unique_id: i32,
                                            unit_index: i32) -> FindTargetUnitIdByIndexRequest {
        FindTargetUnitIdByIndexRequest::new(
            account_unique_id, unit_index)
    }

    pub fn to_summary_passive_skill_request(&self,
                                            unit_card_id: i32) -> SummaryPassiveSkillEffectRequest {
        SummaryPassiveSkillEffectRequest::new(
            unit_card_id)
    }

    pub fn to_find_opponent_by_account_id_request(&self,
                                                  account_unique_id: i32) -> FindOpponentByAccountIdRequest {
        FindOpponentByAccountIdRequest::new(
            account_unique_id)
    }

    pub fn to_attack_target_unit_with_extra_effect_request(&self,
                                                           opponent_unique_id: i32,
                                                           damage: i32,
                                                           extra_status_effect_list: &Vec<ExtraStatusEffect>,
                                                           target_unit_index: i32) -> AttackTargetUnitWithExtraEffectRequest {
        AttackTargetUnitWithExtraEffectRequest::new(
            opponent_unique_id, damage, extra_status_effect_list.clone(), target_unit_index)
    }

    pub fn to_judge_death_of_unit_request(&self,
                                          account_unique_id: i32,
                                          unit_index: i32) -> JudgeDeathOfUnitRequest {
        JudgeDeathOfUnitRequest::new(
            account_unique_id, unit_index)
    }

    pub fn to_place_dead_unit_to_tomb_request(&self,
                                    account_unique_id: i32,
                                    dead_unit_card_id: i32) -> PlaceToTombRequest {
        PlaceToTombRequest::new(
            account_unique_id, dead_unit_card_id)
    }
}
