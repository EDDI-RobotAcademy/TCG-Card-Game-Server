use async_trait::async_trait;

#[async_trait]
pub trait BattleFinishService {
    async fn battle_finish_for_player_battle(&self);
}