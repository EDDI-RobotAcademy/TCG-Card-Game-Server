use async_trait::async_trait;
use crate::battle_wait_queue::service::request::battle_wait_queue_request::BattleWaitQueueRequest;
use crate::battle_wait_queue::service::response::battle_wait_queue_response::BattleWaitQueueResponse;


#[async_trait]
pub trait BattleWaitQueueService {
    async fn enqueue_player_id_to_wait_queue(&self, battle_wait_queue_request: BattleWaitQueueRequest) -> BattleWaitQueueResponse;
}