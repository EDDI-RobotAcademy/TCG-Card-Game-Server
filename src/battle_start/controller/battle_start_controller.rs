use async_trait::async_trait;

use crate::battle_start::controller::request_form::battle_start_request_form::BattleStartRequestForm;
use crate::battle_start::controller::response_form::battle_start_response_form::BattleStartResponseForm;

#[async_trait]
pub trait BattleStartController {
    async fn request_to_start_battle(
        &self, battle_start_request_form: BattleStartRequestForm)
        -> BattleStartResponseForm;
}
