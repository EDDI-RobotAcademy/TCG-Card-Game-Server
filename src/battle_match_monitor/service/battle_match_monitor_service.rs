use async_trait::async_trait;

#[async_trait]
pub trait BattleMatchMonitorService {
    async fn check_battle_match(&self);
}