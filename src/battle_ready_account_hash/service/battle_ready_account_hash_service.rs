use async_trait::async_trait;

use crate::battle_ready_account_hash::service::request::battle_ready_account_hash_request::BattleReadyAccountHashRequest;
use crate::battle_ready_account_hash::service::request::check_battle_prepare_request::CheckBattlePrepareRequest;
use crate::battle_ready_account_hash::service::response::battle_ready_account_hash_response::BattleReadyAccountHashResponse;
use crate::battle_ready_account_hash::service::response::check_battle_prepare_response::CheckBattlePrepareResponse;

#[async_trait]
pub trait BattleReadyAccountHashService {
    // async fn start_monitor(&mut self);
    async fn check_ready_for_battle(&self, battle_ready_account_hash_request: BattleReadyAccountHashRequest) -> BattleReadyAccountHashResponse;
    async fn check_prepare_for_battle(&self, check_battle_prepare_request: CheckBattlePrepareRequest) -> CheckBattlePrepareResponse;
}