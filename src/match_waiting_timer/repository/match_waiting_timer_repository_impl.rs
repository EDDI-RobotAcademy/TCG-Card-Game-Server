use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;

use crate::match_waiting_timer::entity::match_waiting_timer::MatchWaitingTimer;
use crate::match_waiting_timer::repository::match_waiting_timer_repository::MatchWaitingTimerRepository;
use crate::receiver::entity::receive_data::ReceiveData;
use crate::receiver::repository::server_receiver_repository::ServerReceiverRepository;

pub struct MatchWaitingTimerRepositoryImpl {
    match_waiting_timer: MatchWaitingTimer,
}

impl MatchWaitingTimerRepositoryImpl {
    pub fn new() -> Self {
        MatchWaitingTimerRepositoryImpl {
            match_waiting_timer: MatchWaitingTimer::new(),
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<MatchWaitingTimerRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<MatchWaitingTimerRepositoryImpl>> =
                Arc::new(AsyncMutex::new(MatchWaitingTimerRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl MatchWaitingTimerRepository for MatchWaitingTimerRepositoryImpl {
    async fn set_match_waiting_timer(&mut self, account_unique_id: i32) {
        println!("MatchWaitingTimerRepositoryImpl: set_match_waiting_timer()");

        self.match_waiting_timer.start_timer(account_unique_id);
    }
}