use async_trait::async_trait;
use crate::battle_room::service::request::battle_match_request::BattleMatchRequest;
use crate::battle_room::service::response::battle_match_response::BattleMatchResponse;

#[async_trait]
pub trait BattleRoomService {
    // async fn enqueue_player_id_to_wait_queue(&self, battle_match_request: BattleMatchRequest) -> BattleMatchResponse;
}