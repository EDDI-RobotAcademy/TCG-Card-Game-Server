use async_trait::async_trait;
use crate::account_point::entity::account_point::AccountPoint;

#[async_trait]
pub trait AccountPointRepository {
    async fn update_gold(&self, account_points: AccountPoint, gold: i32) -> Result<usize, diesel::result::Error>;
    // async fn pay_gold(&self, account: AccountPoint, account_user_id: &str, gain_golds: i32) -> Result<usize, diesel::result::Error>;
}