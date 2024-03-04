use async_trait::async_trait;
use crate::game_field_unit_action_possibility_validator::service::request::is_unit_basic_attack_possible_request::IsUnitBasicAttackPossibleRequest;
use crate::game_field_unit_action_possibility_validator::service::request::is_using_active_skill_possible_request::IsUsingActiveSkillPossibleRequest;
use crate::game_field_unit_action_possibility_validator::service::request::is_using_deploy_passive_skill_possible_request::IsUsingDeployPassiveSkillPossibleRequest;
use crate::game_field_unit_action_possibility_validator::service::request::is_using_turn_start_passive_skill_request::IsUsingTurnStartPassiveSkillPossibleRequest;
use crate::game_field_unit_action_possibility_validator::service::response::is_unit_basic_attack_possible_response::IsUnitBasicAttackPossibleResponse;
use crate::game_field_unit_action_possibility_validator::service::response::is_using_active_skill_possible_response::IsUsingActiveSkillPossibleResponse;
use crate::game_field_unit_action_possibility_validator::service::response::is_using_deploy_passive_skill_possible_response::IsUsingDeployPassiveSkillPossibleResponse;
use crate::game_field_unit_action_possibility_validator::service::response::is_using_turn_start_passive_skill_response::IsUsingTurnStartPassiveSkillPossibleResponse;

#[async_trait]
pub trait GameFieldUnitActionPossibilityValidatorService {
    async fn is_unit_basic_attack_possible(
        &self, is_unit_action_possible_request: IsUnitBasicAttackPossibleRequest)
        -> IsUnitBasicAttackPossibleResponse;

    async fn is_using_active_skill_possible(
        &self, is_using_active_skill_possible_request: IsUsingActiveSkillPossibleRequest)
        -> IsUsingActiveSkillPossibleResponse;
    async fn is_using_deploy_passive_skill_possible(
        &self, is_using_deploy_passive_skill_possible_request: IsUsingDeployPassiveSkillPossibleRequest)
        -> IsUsingDeployPassiveSkillPossibleResponse;
    async fn is_using_turn_start_passive_skill_possible(
        &self, is_using_turn_start_passive_skill_possible_request: IsUsingTurnStartPassiveSkillPossibleRequest)
        -> IsUsingTurnStartPassiveSkillPossibleResponse;
}