use async_trait::async_trait;
use crate::account_point::entity::account_point::account_points::account_id;
use crate::account_point::entity::account_point::AccountPoint;

#[async_trait]
pub trait AccountPointRepository {
    async fn find_by_account_id(&self, account_user_id: i32) -> Result<Option<AccountPoint>, diesel::result::Error>;
    async fn update_gold(&self, account_point: AccountPoint, gold: i32) -> Result<usize, diesel::result::Error>;
    // async fn pay_gold(&self, account: AccountPoint, account_user_id: &str, gain_golds: i32) -> Result<usize, diesel::result::Error>;
}