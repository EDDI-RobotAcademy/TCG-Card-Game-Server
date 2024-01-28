use async_trait::async_trait;
use crate::account::entity::account::Account;

#[async_trait]
pub trait AccountRepository {
    async fn save(&self, account: Account) -> Result<(), diesel::result::Error>;
    async fn find_by_user_id(&self, account_user_id: &str) -> Result<Option<Account>, diesel::result::Error>;
    async fn delete(&self, account: Account) -> Result<(), diesel::result::Error>;
    async fn update(&self, account: Account, account_new_password: &str) -> Result<usize, diesel::result::Error> ;
}