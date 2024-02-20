use async_trait::async_trait;
use crate::game_field_unit_action_possibility_validator::service::request::is_unit_basic_attack_possible_request::IsUnitBasicAttackPossibleRequest;
use crate::game_field_unit_action_possibility_validator::service::response::is_unit_basic_attack_possible_response::IsUnitBasicAttackPossibleResponse;

#[async_trait]
pub trait GameFieldUnitActionValidatorService {
    async fn is_unit_basic_attack_possible(
        &self, is_unit_action_possible_request: IsUnitBasicAttackPossibleRequest)
        -> IsUnitBasicAttackPossibleResponse;
}