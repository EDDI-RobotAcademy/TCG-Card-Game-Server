use async_trait::async_trait;
use crate::battle_ready_monitor::service::request::battle_ready_request::BattleReadyRequest;
use crate::battle_ready_monitor::service::response::battle_ready_response::BattleReadyResponse;

#[async_trait]
pub trait BattleReadyMonitorService {
    // async fn start_monitor(&mut self);
    async fn check_ready_for_battle(&self, battle_ready_request: BattleReadyRequest) -> BattleReadyResponse;
}