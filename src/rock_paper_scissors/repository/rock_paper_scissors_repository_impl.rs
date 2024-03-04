use std::sync::Arc;
use std::error::Error;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;

use crate::rock_paper_scissors::entity::rock_paper_scissors_wait_hash::RockPaperScissorsWaitHash;
use crate::rock_paper_scissors::repository::rock_paper_scissors_repository::RockPaperScissorsRepository;


pub struct RockPaperScissorsRepositoryImpl {
    wait_hashmap: Arc<AsyncMutex<RockPaperScissorsWaitHash>>,
}

impl RockPaperScissorsRepositoryImpl {
    pub fn new() -> Self {
        RockPaperScissorsRepositoryImpl {
            wait_hashmap: Arc::new(AsyncMutex::new(RockPaperScissorsWaitHash::new())),
        }
    }

    pub fn get_wait_hashmap(&self) -> Arc<AsyncMutex<RockPaperScissorsWaitHash>> {
        Arc::clone(&self.wait_hashmap)
    }

    pub fn get_instance() -> Arc<AsyncMutex<RockPaperScissorsRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<RockPaperScissorsRepositoryImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        RockPaperScissorsRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl RockPaperScissorsRepository for RockPaperScissorsRepositoryImpl {
    async fn register_choice_repo(&self, account_unique_id: i32, choice: String) -> bool {
        println!("RockPaperScissorsRepositoryImpl: register_choice_repo()");

        let mut waiting_hashmap_guard = self.wait_hashmap.lock().await;
        waiting_hashmap_guard.save_choice(account_unique_id, choice).await;

        drop(waiting_hashmap_guard);

        true
    }

    async fn change_draw_choices_repo(&self, account_unique_id: i32, opponent_unique_id: i32) -> bool {
        println!("RockPaperScissorsRepositoryImpl: change_draw_choices_repo()");

        let waiting_hashmap_guard = self.wait_hashmap.lock().await;
        waiting_hashmap_guard.change_draw_choices(account_unique_id, opponent_unique_id).await;

        drop(waiting_hashmap_guard);

        true
    }

    async fn check_opponent_choice_repo(&self, opponent_unique_id: i32) -> Result<bool, Box<dyn Error>> {
        println!("RockPaperScissorsRepositoryImpl: check_opponent_choice_repo()");

        let waiting_hashmap_guard = self.wait_hashmap.lock().await;
        let mut check_result = waiting_hashmap_guard.check_opponent_choice(opponent_unique_id).await;

        drop(waiting_hashmap_guard);

        Ok(check_result.unwrap())
    }

    async fn clear_choices_repo(&self, account_unique_id: i32) -> bool {
        println!("RockPaperScissorsRepositoryImpl: clear_choices_repo()");

        let mut waiting_hashmap_guard = self.wait_hashmap.lock().await;
        waiting_hashmap_guard.clear_choice(account_unique_id).await;

        drop(waiting_hashmap_guard);

        true
    }
}

