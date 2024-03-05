use std::sync::Arc;
use std::time::Duration;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;
use crate::action_waiting_timer::repository::action_waiting_timer_repository::ActionWaitingTimerRepository;
use crate::action_waiting_timer::repository::action_waiting_timer_repository_impl::ActionWaitingTimerRepositoryImpl;
use crate::action_waiting_timer::service::action_waiting_timer_service::ActionWaitingTimerService;
use crate::action_waiting_timer::service::request::action_waiting_timer_request::ActionWaitingTimerRequest;
use crate::action_waiting_timer::service::response::action_waiting_timer_response::ActionWaitingTimerResponse;
use crate::battle_wait_queue::service::response::battle_match_cancel_response::BattleMatchCancelResponse;
use crate::redis::repository::redis_in_memory_repository::RedisInMemoryRepository;
use crate::redis::repository::redis_in_memory_repository_impl::RedisInMemoryRepositoryImpl;

pub struct ActionWaitingTimerServiceImpl {
    action_waiting_timer_repository: Arc<AsyncMutex<ActionWaitingTimerRepositoryImpl>>,
}

impl ActionWaitingTimerServiceImpl {
    pub fn new(
               action_waiting_timer_repository: Arc<AsyncMutex<ActionWaitingTimerRepositoryImpl>>,
    ) -> Self {

        ActionWaitingTimerServiceImpl {
            action_waiting_timer_repository
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<ActionWaitingTimerServiceImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<ActionWaitingTimerServiceImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        ActionWaitingTimerServiceImpl::new(
                            ActionWaitingTimerRepositoryImpl::get_instance())));
        }
        INSTANCE.clone()
    }

}

#[async_trait]
impl ActionWaitingTimerService for ActionWaitingTimerServiceImpl {



    async fn set_action_waiting_time(&self, action_waiting_timer_request: ActionWaitingTimerRequest) -> ActionWaitingTimerResponse {
        println!("ActionWaitingTimerServiceImpl: measure_action_waiting_time()");
        let account_unique_id = action_waiting_timer_request.get_account_unique_id();

        let mut action_waiting_timer_repository = self.action_waiting_timer_repository.lock().await;


        let mut response=action_waiting_timer_repository.set_action_waiting_timer(account_unique_id, Duration::from_secs(60)).await;



        if response==true {
            return ActionWaitingTimerResponse::new(true)
        }

        return ActionWaitingTimerResponse::new(false)
    }
}