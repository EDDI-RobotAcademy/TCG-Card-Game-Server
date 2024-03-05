use std::time::Duration;
use async_trait::async_trait;
use crate::rock_paper_scissors::entity::rock_paper_scissors_result::RockPaperScissorsResult;

#[async_trait]
pub trait RockPaperScissorsRepository {
    async fn register_choice_repo(&self, account_unique_id: i32, choice: String) -> bool;
    async fn change_draw_choices_repo(&self, account_unique_id: i32, opponent_unique_id: i32) -> bool;
    async fn check_result_repo(&self, account_unique_id: i32, opponent_unique_id: i32) -> RockPaperScissorsResult;

}