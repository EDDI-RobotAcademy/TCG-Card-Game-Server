use crate::battle_room::service::request::find_opponent_by_account_id_request::FindOpponentByAccountIdRequest;
use crate::common::card_attributes::card_grade::card_grade_enum::GradeEnum;
use crate::common::card_attributes::card_race::card_race_enum::RaceEnum;
use crate::game_card_unit::service::request::summary_unit_card_info_request::SummaryUnitCardInfoRequest;
use crate::game_field_unit::service::request::add_unit_to_game_field_request::AddUnitToGameFieldRequest;
use crate::game_hand::service::request::use_game_hand_unit_card_request::UseGameHandUnitCardRequest;
use crate::game_protocol_validation::service::request::check_protocol_hacking_request::CheckProtocolHackingRequest;
use crate::game_protocol_validation::service::request::is_it_unit_card_request::IsItUnitCardRequest;
use crate::notify_player_action::service::request::notify_to_opponent_you_deploy_unit_request::NotifyToOpponentYouDeployUnitRequest;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;

pub struct DeployUnitRequestForm{
    session_id: String,
    unit_id: String,
}

impl DeployUnitRequestForm {
    pub fn new(session_id: String, unit_id: String) -> Self {
        DeployUnitRequestForm {
            session_id: session_id.to_string(),
            unit_id: unit_id.to_string(),
        }
    }

    pub fn get_session_id(&self) -> &str {
        &self.session_id
    }

    pub fn get_unit_id(&self) -> &str {
        &self.unit_id
    }

    pub fn to_session_validation_request(&self) -> GetValueWithKeyRequest {
        GetValueWithKeyRequest::new(self.session_id.clone().as_str())
    }

    pub fn to_check_protocol_hacking_request(&self, account_unique_id: i32, unit_card_id: i32) -> CheckProtocolHackingRequest {
        CheckProtocolHackingRequest::new(account_unique_id, unit_card_id)
    }

    pub fn to_is_it_unit_card_request(&self, unit_card_id: i32) -> IsItUnitCardRequest {
        IsItUnitCardRequest::new(unit_card_id)
    }

    pub fn to_use_game_hand_unit_card_request(&self, account_unique_id: i32, unit_card_id: i32) -> UseGameHandUnitCardRequest {
        UseGameHandUnitCardRequest::new(
            account_unique_id, unit_card_id)
    }

    pub fn to_summary_unit_card_info_request(&self, unit_card_id: i32) -> SummaryUnitCardInfoRequest {
        SummaryUnitCardInfoRequest::new(unit_card_id)
    }

    pub fn to_add_unit_to_game_field_request(&self,
                                             account_unique_id: i32,
                                             unit_card_id: i32,
                                             unit_race: RaceEnum,
                                             unit_grade: GradeEnum,
                                             unit_attack_point: i32,
                                             unit_health_point: i32,
                                             unit_attack_required_energy: i32,
                                             first_passive_skill: bool,
                                             second_passive_skill: bool,
                                             third_passive_skill: bool) -> AddUnitToGameFieldRequest {

        AddUnitToGameFieldRequest::new(
            account_unique_id,
            unit_card_id,
            unit_race,
            unit_grade,
            unit_attack_point,
            unit_health_point,
            unit_attack_required_energy,
            first_passive_skill,
            second_passive_skill,
            third_passive_skill)
    }

    pub fn to_find_opponent_by_account_id_request(&self, account_unique_id: i32) -> FindOpponentByAccountIdRequest {
        FindOpponentByAccountIdRequest::new(
            account_unique_id)
    }

    pub fn to_notify_to_opponent_what_you_do_request(&self, opponent_unique_id: i32, usage_hand_card_id: i32) -> NotifyToOpponentYouDeployUnitRequest {
        NotifyToOpponentYouDeployUnitRequest::new(
            opponent_unique_id, usage_hand_card_id)
    }
}