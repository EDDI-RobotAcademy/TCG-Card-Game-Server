use std::sync::Arc;
use std::time::Duration;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::action_waiting_timer::entity::action_waiting_timer::ActionWaitingTimer;
use crate::action_waiting_timer::repository::action_waiting_timer_repository::ActionWaitingTimerRepository;


pub struct ActionWaitingTimerRepositoryImpl {
    action_waiting_timer: ActionWaitingTimer,
}

impl ActionWaitingTimerRepositoryImpl {
    pub fn new() -> Self {
        ActionWaitingTimerRepositoryImpl {
            action_waiting_timer: ActionWaitingTimer::new(),
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<ActionWaitingTimerRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<ActionWaitingTimerRepositoryImpl>> =
                Arc::new(AsyncMutex::new(ActionWaitingTimerRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl ActionWaitingTimerRepository for ActionWaitingTimerRepositoryImpl {


    async fn set_action_waiting_timer(&mut self, opponent_id: i32, duration: Duration) -> bool {
        println!("ActionWaitingTimerRepositoryImpl: set_action_waiting_timer()");

        self.action_waiting_timer.start_timer(opponent_id);

        return true
    }
}