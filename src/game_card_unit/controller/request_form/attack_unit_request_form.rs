use crate::battle_room::service::request::find_opponent_by_account_id_request::FindOpponentByAccountIdRequest;
use crate::game_field_unit::entity::extra_status_effect::ExtraStatusEffect;
use crate::game_field_unit::service::request::acquire_unit_attack_point_request::AcquireUnitAttackPointRequest;
use crate::game_field_unit::service::request::acquire_unit_extra_effect_request::AcquireUnitExtraEffectRequest;
use crate::game_field_unit::service::request::attack_target_unit_with_extra_effect_request::AttackTargetUnitWithExtraEffectRequest;
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

    pub fn to_acquire_unit_attack_point_request(&self, account_unique_id: i32, attacker_unit_card_index: i32) -> AcquireUnitAttackPointRequest {
        AcquireUnitAttackPointRequest::new(account_unique_id, attacker_unit_card_index)
    }

    pub fn to_acquire_unit_extra_effect_request(&self, account_unique_id: i32, attacker_unit_card_index: i32) -> AcquireUnitExtraEffectRequest {
        AcquireUnitExtraEffectRequest::new(account_unique_id, attacker_unit_card_index)
    }

    pub fn to_find_opponent_by_account_id_request(&self, account_unique_id: i32) -> FindOpponentByAccountIdRequest {
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

}
