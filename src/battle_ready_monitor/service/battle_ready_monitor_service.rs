use async_trait::async_trait;

#[async_trait]
pub trait BattleReadyMonitorService {
    async fn start_monitor(&mut self);
}