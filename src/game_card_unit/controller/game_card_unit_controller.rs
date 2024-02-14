use async_trait::async_trait;
use crate::game_card_unit::controller::request_form::attack_unit_request_form::AttackUnitRequestForm;
use crate::game_card_unit::controller::request_form::deploy_unit_request_form::DeployUnitRequestForm;
use crate::game_card_unit::controller::response_form::attack_unit_response_form::AttackUnitResponseForm;
use crate::game_card_unit::controller::response_form::deploy_unit_response_form::DeployUnitResponseForm;

#[async_trait]
pub trait GameCardUnitController {
    async fn request_to_deploy_unit(
        &self, deploy_unit_request_form: DeployUnitRequestForm) -> DeployUnitResponseForm;
    async fn request_to_attack_unit(&self, attack_unit_request_form: AttackUnitRequestForm) -> AttackUnitResponseForm;
}