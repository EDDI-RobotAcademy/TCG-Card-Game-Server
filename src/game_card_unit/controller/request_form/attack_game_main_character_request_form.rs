use crate::battle_room::service::request::find_opponent_by_account_id_request::FindOpponentByAccountIdRequest;
use crate::game_card_unit::service::request::summary_unit_card_info_request::SummaryUnitCardInfoRequest;
use crate::game_field_unit::service::request::execute_turn_action_request::ExecuteTurnActionRequest;
use crate::game_field_unit::service::request::acquire_unit_attack_point_request::AcquireUnitAttackPointRequest;
use crate::game_field_unit::service::request::acquire_unit_extra_effect_request::AcquireUnitExtraEffectRequest;

// 플레이어 데미지
use crate::game_main_character::service::request::check_main_character_of_account_unique_id_request::CheckMainCharacterOfAccountUniqueIdRequest;
use crate::game_main_character::service::request::apply_damage_to_main_character_request::ApplyDamageToMainCharacterRequest;
use crate::game_field_unit::service::request::find_target_unit_id_by_index_request::FindTargetUnitIdByIndexRequest;
use crate::game_field_unit_action_possibility_validator::service::request::is_unit_basic_attack_possible_request::IsUnitBasicAttackPossibleRequest;
use crate::game_protocol_validation::service::request::is_this_your_turn_request::IsThisYourTurnRequest;
use crate::game_winner_check::service::request::check_main_character_request::CheckMainCharacterRequest;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;

pub struct AttackGameMainCharacterRequestForm {
    session_id: String,
    attacker_unit_index: String,
    target_game_main_character_index: String,
}

impl AttackGameMainCharacterRequestForm {
    pub fn new(session_id: String, attacker_unit_index: String, target_game_main_character_index: String) -> Self {
        AttackGameMainCharacterRequestForm {
            session_id: session_id.to_string(),
            attacker_unit_index: attacker_unit_index.to_string(),
            target_game_main_character_index: target_game_main_character_index.to_string(),
        }
    }

    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }

    pub fn get_attacker_unit_index(&self) -> &str {
        &self.attacker_unit_index
    }

    pub fn get_target_game_main_character_index(&self) -> &str {
        &self.target_game_main_character_index
    }

    pub fn to_session_validation_request(&self) -> GetValueWithKeyRequest {
        GetValueWithKeyRequest::new(self.session_id.clone().as_str())
    }

    pub fn to_is_this_your_turn_request(&self,
                                        account_unique_id: i32) -> IsThisYourTurnRequest {
        IsThisYourTurnRequest::new(account_unique_id)
    }

    pub fn to_summary_unit_card_info_request(&self,
                                             unit_card_id: i32) -> SummaryUnitCardInfoRequest {
        SummaryUnitCardInfoRequest::new(unit_card_id)
    }

    pub fn to_is_unit_basic_attack_possible_request(&self,
                                                    account_unique_id: i32,
                                                    field_unit_index: i32,
                                                    basic_attack_required_energy_count: i32) -> IsUnitBasicAttackPossibleRequest {
        IsUnitBasicAttackPossibleRequest::new(
            account_unique_id, field_unit_index, basic_attack_required_energy_count)
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

    pub fn to_find_opponent_by_account_id_request(&self,
                                                  account_unique_id: i32) -> FindOpponentByAccountIdRequest {
        FindOpponentByAccountIdRequest::new(
            account_unique_id)
    }




    pub fn to_check_main_character_of_account_unique_id_request(&self,
                                                                account_unique_id: i32) -> CheckMainCharacterOfAccountUniqueIdRequest {
        CheckMainCharacterOfAccountUniqueIdRequest::new(
            account_unique_id)
    }
    pub fn to_apply_damage_to_main_character_request(&self,
                                                     account_unique_id: i32,
                                                     damage: i32) -> ApplyDamageToMainCharacterRequest {
        ApplyDamageToMainCharacterRequest::new(
            account_unique_id, damage)
    }
    pub fn to_check_health_of_main_character_for_setting_game_winner_request(&self,
                                                                            account_unique_id: i32,
                                                                            opponent_unique_id: i32) -> CheckMainCharacterRequest {
        CheckMainCharacterRequest::new(account_unique_id, opponent_unique_id)
    }
}