use async_trait::async_trait;

#[async_trait]
pub trait BattlePrepareTaskService {
    // TODO: 추후 1 대 1 배틀, 다 대 다, 레이드 등등을 고려할 필요가 있음 (네이밍)
    async fn prepare_for_player_battle(&self);
}