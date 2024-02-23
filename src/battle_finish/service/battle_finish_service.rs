use async_trait::async_trait;
use crate::battle_finish::service::request::battle_finish_request::BattleFinishRequest;
use crate::battle_finish::service::response::battle_finish_response::BattleFinishResponse;

#[async_trait]
pub trait BattleFinishService {
    async fn battle_finish_for_player_battle(&self, battle_finish_request: BattleFinishRequest) -> BattleFinishResponse;
}