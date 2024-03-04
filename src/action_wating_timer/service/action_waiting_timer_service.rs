use async_trait::async_trait;
use crate::action_wating_timer::service::request::action_waiting_timer_request::ActionWaitingTimerRequest;
use crate::action_wating_timer::service::response::action_waiting_timer_response::ActionWaitingTimerResponse;


#[async_trait]
pub trait ActionWaitingTimerService {
    async fn set_action_waiting_time(&self, action_waiting_timer_request: ActionWaitingTimerRequest) -> ActionWaitingTimerResponse;

}