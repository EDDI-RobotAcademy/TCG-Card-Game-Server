use std::collections::HashMap;
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
use crate::game_main_character::entity::status_main_character::StatusMainCharacterEnum;
use crate::game_protocol_validation::service::request::is_this_your_turn_request::IsThisYourTurnRequest;
use crate::game_winner_check::service::request::check_main_character_request::CheckMainCharacterRequest;
use crate::notify_player_action_info::service::request::notice_basic_attack_to_main_character_request::NoticeBasicAttackToMainCharacterRequest;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;
use crate::ui_data_generator::entity::player_index_enum::PlayerIndex;
use crate::ui_data_generator::service::request::generate_opponent_main_character_health_point_data_request::GenerateOpponentMainCharacterHealthPointDataRequest;
use crate::ui_data_generator::service::request::generate_opponent_main_character_survival_data_request::GenerateOpponentMainCharacterSurvivalDataRequest;

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

    pub fn to_check_main_character_of_account_unique_id_request(
        &self,
        account_unique_id: i32) -> CheckMainCharacterOfAccountUniqueIdRequest {

        CheckMainCharacterOfAccountUniqueIdRequest::new(
            account_unique_id)
    }

    pub fn to_apply_damage_to_main_character_request(
        &self,
        account_unique_id: i32,
        damage: i32) -> ApplyDamageToMainCharacterRequest {

        ApplyDamageToMainCharacterRequest::new(
            account_unique_id, damage)
    }

    pub fn to_check_main_character_for_setting_game_winner_request(
        &self,
        account_unique_id: i32,
        opponent_unique_id: i32) -> CheckMainCharacterRequest {

        CheckMainCharacterRequest::new(account_unique_id, opponent_unique_id)
    }

    pub fn to_generate_opponent_main_character_health_point_data_request(
        &self,
        main_character_health_point: i32
    ) -> GenerateOpponentMainCharacterHealthPointDataRequest {

        GenerateOpponentMainCharacterHealthPointDataRequest::new(
            main_character_health_point)
    }

    pub fn to_generate_opponent_main_character_survival_data_request(
        &self,
        main_character_status: StatusMainCharacterEnum
    ) -> GenerateOpponentMainCharacterSurvivalDataRequest {

        GenerateOpponentMainCharacterSurvivalDataRequest::new(
            main_character_status)
    }

    pub fn to_notice_basic_attack_to_main_character_request(
        &self,
        opponent_unique_id: i32,
        player_main_character_health_point_map_for_notice: HashMap<PlayerIndex, i32>,
        player_main_character_survival_map_for_notice: HashMap<PlayerIndex, StatusMainCharacterEnum>
    ) -> NoticeBasicAttackToMainCharacterRequest {

        NoticeBasicAttackToMainCharacterRequest::new(
            opponent_unique_id,
            player_main_character_health_point_map_for_notice,
            player_main_character_survival_map_for_notice)
    }
}