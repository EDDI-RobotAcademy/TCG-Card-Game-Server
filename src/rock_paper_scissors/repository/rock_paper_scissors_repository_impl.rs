use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;

use crate::rock_paper_scissors::entity::rock_paper_scissors_result::RockPaperScissorsResult;
use crate::rock_paper_scissors::entity::rock_paper_scissors_result::RockPaperScissorsResult::*;
use crate::rock_paper_scissors::entity::rock_paper_scissors_result_hash::RockPaperScissorsResultHash;

use crate::rock_paper_scissors::entity::rock_paper_scissors_wait_hash::RockPaperScissorsWaitHash;
use crate::rock_paper_scissors::repository::rock_paper_scissors_repository::RockPaperScissorsRepository;


pub struct RockPaperScissorsRepositoryImpl {
    wait_hashmap: Arc<AsyncMutex<RockPaperScissorsWaitHash>>,
    result_hashmap: Arc<AsyncMutex<RockPaperScissorsResultHash>>,
}

impl RockPaperScissorsRepositoryImpl {
    pub fn new() -> Self {
        RockPaperScissorsRepositoryImpl {
            wait_hashmap: Arc::new(AsyncMutex::new(RockPaperScissorsWaitHash::new())),
            result_hashmap: Arc::new(AsyncMutex::new(RockPaperScissorsResultHash::new())),
        }
    }

    pub fn get_wait_hashmap(&self) -> Arc<AsyncMutex<RockPaperScissorsWaitHash>> {
        Arc::clone(&self.wait_hashmap)
    }

    pub fn get_result_hashmap(&self) -> Arc<AsyncMutex<RockPaperScissorsResultHash>> {
        Arc::clone(&self.result_hashmap)
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

        // 과거 가위바위보 결과 기록 삭제
        let mut result_hashmap_guard = self.result_hashmap.lock().await;
        result_hashmap_guard.remove_result(account_unique_id).await;

        drop(result_hashmap_guard);

        true
    }

    async fn change_draw_choices_repo(&self, account_unique_id: i32, opponent_unique_id: i32) -> bool {
        println!("RockPaperScissorsRepositoryImpl: change_draw_choices_repo()");

        let waiting_hashmap_guard = self.wait_hashmap.lock().await;
        waiting_hashmap_guard.change_draw_choices(account_unique_id, opponent_unique_id).await;

        drop(waiting_hashmap_guard);

        true
    }

    async fn check_result_repo(&self, account_unique_id: i32, opponent_unique_id: i32) -> RockPaperScissorsResult {
        println!("RockPaperScissorsRepositoryImpl: check_result_repo()");

        let mut result_hashmap_guard = self.result_hashmap.lock().await;
        let my_result = result_hashmap_guard.get_result(account_unique_id).await;

        // 1. 내 결과가 이미 나와 있는 경우
        if my_result.is_some() {
            match my_result.unwrap() {
                WIN => {
                    result_hashmap_guard.save_result(opponent_unique_id, LOSE).await;
                    return WIN
                },
                LOSE => {
                    result_hashmap_guard.save_result(opponent_unique_id, WIN).await;
                    return LOSE
                }
                _ => {}
            }
        }

        // 2. 상대와 내 가위바위보 choice 가 모두 존재하는 경우
        let mut waiting_hashmap_guard = self.wait_hashmap.lock().await;
        let my_choice = waiting_hashmap_guard.get_choice(account_unique_id).await;
        let opponent_choice = waiting_hashmap_guard.get_choice(opponent_unique_id).await;

        if opponent_choice.is_none() {
            return WAIT
        }

        return match (my_choice.unwrap().as_str(), opponent_choice.unwrap().as_str()) {
            ("Rock", "Scissors") | ("Paper", "Rock") | ("Scissors", "Paper") => {
                waiting_hashmap_guard.remove_choice(opponent_unique_id).await;
                result_hashmap_guard.save_result(opponent_unique_id, LOSE).await;
                WIN
            },
            ("Scissors", "Rock") | ("Rock", "Paper") | ("Paper", "Scissors") => {
                waiting_hashmap_guard.remove_choice(opponent_unique_id).await;
                result_hashmap_guard.save_result(opponent_unique_id, WIN).await;
                LOSE
            },
            _ => WAIT
        }
    }
}