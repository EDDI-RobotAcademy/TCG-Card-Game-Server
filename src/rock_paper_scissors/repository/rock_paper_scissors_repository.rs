use std::error::Error;
use async_trait::async_trait;

#[async_trait]
pub trait RockPaperScissorsRepository {
    async fn register_choice_repo(&self, account_unique_id: i32, choice: String) -> bool;
    async fn change_draw_choices_repo(&self, account_unique_id: i32, opponent_unique_id: i32) -> bool;
    async fn check_opponent_choice_repo(&self, opponent_unique_id: i32) -> Result<bool, Box<dyn Error>>;
    async fn clear_choices_repo(&self, account_unique_id: i32) -> bool;
}