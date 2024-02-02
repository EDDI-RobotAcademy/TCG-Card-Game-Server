use async_trait::async_trait;
use crate::account_point::entity::account_point::account_points::account_id;
use crate::account_point::entity::account_point::AccountPoint;

#[async_trait]
pub trait AccountPointRepository {
    async fn set_account_point(&self, account_id: i32, golds: i32) -> AccountPoint;
    async fn save_account_points(&self, account_point: AccountPoint) -> Result<(), diesel::result::Error>;
    async fn find_by_account_id(&self, account_user_id: i32) -> Result<Option<AccountPoint>, diesel::result::Error>;
    async fn update_gold(&self, account_point: AccountPoint, gold: i32) -> Result<usize, diesel::result::Error>;
    async fn delete_account_points(&self, account_id: i32) -> Result<(), diesel::result::Error>;
}