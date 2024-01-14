use async_trait::async_trait;
use crate::account::entity::account::Account;

#[async_trait]
pub trait AccountRepository {
    async fn save(&self, account: Account);
}