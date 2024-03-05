use std::sync::Arc;
use std::time::Duration;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;
use crate::mulligan::entity::mulligan_finished_player_list::MulliganFinishedPlayerList;
use crate::mulligan::entity::mulligan_timer_hash::MulliganTimerHash;
use crate::mulligan::repository::mulligan_repository::MulliganRepository;

pub struct MulliganRepositoryImpl {
    mulligan_finished_player_list: Arc<AsyncMutex<MulliganFinishedPlayerList>>,
    mulligan_timer_hash: Arc<AsyncMutex<MulliganTimerHash>>,
}

impl MulliganRepositoryImpl {
    pub fn new() -> Self {
        MulliganRepositoryImpl {
            mulligan_finished_player_list: Arc::new(AsyncMutex::new(MulliganFinishedPlayerList::new())),
            mulligan_timer_hash: Arc::new(AsyncMutex::new(MulliganTimerHash::new())),
        }
    }

    pub fn get_mulligan_finished_player_list(&self) -> Arc<AsyncMutex<MulliganFinishedPlayerList>> {
        Arc::clone(&self.mulligan_finished_player_list)
    }

    pub fn get_mulligan_timer_hash(&self) -> Arc<AsyncMutex<MulliganTimerHash>> {
        Arc::clone(&self.mulligan_timer_hash)
    }

    pub fn get_instance() -> Arc<AsyncMutex<MulliganRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<MulliganRepositoryImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        MulliganRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl MulliganRepository for MulliganRepositoryImpl {
    async fn set_mulligan_timer(&self, account_unique_id: i32) -> bool {
        println!("MulliganRepositoryImpl: set_mulligan_timer");

        let mut mulligan_timer_hash_guard = self.mulligan_timer_hash.lock().await;
        mulligan_timer_hash_guard.start(account_unique_id).await;

        true
    }

    async fn check_opponent_mulligan_timer_over(&self, opponent_unique_id: i32) -> bool {
        println!("MulliganRepositoryImpl: check_opponent_mulligan_timer_over");

        let mut mulligan_timer_hash_guard = self.mulligan_timer_hash.lock().await;

        mulligan_timer_hash_guard.check(opponent_unique_id, Duration::from_secs(30)).await.unwrap()
    }

    async fn record_mulligan_finish(&self, account_unique_id: i32) -> bool {
        println!("MulliganRepositoryImpl: record_mulligan_finish");

        let mut mulligan_finished_player_list_guard =
            self.mulligan_finished_player_list.lock().await;

        mulligan_finished_player_list_guard.set(account_unique_id).await;

        true
    }

    async fn check_opponent_mulligan_finish(&self, opponent_unique_id: i32) -> bool {
        println!("MulliganRepositoryImpl: check_opponent_mulligan_finish");

        let mut mulligan_finished_player_list_guard =
            self.mulligan_finished_player_list.lock().await;

        let is_finished =
            mulligan_finished_player_list_guard.check(opponent_unique_id).await.unwrap();

        if is_finished {
            let mut mulligan_timer_hash_guard = self.mulligan_timer_hash.lock().await;
            mulligan_timer_hash_guard.remove(opponent_unique_id).await;
        }

        is_finished
    }
}