use async_trait::async_trait;
use crate::first_turn_decision_wait_queue::service::request::first_turn_decision_wait_queue_request::FirstTurnDecisionWaitQueueRequest;
use crate::first_turn_decision_wait_queue::service::response::first_turn_decision_wait_queue_response::FirstTurnDecisionWaitQueueResponse;


#[async_trait]
pub trait FirstTurnDecisionWaitQueueService {
    async fn enqueue_player_tuple_to_wait_queue(&self, first_turn_decision_wait_queue_request: FirstTurnDecisionWaitQueueRequest) -> FirstTurnDecisionWaitQueueResponse;
}