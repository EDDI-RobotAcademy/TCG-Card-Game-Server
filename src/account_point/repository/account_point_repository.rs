#[async_trait]
pub trait AccountPointRepository {
    async fn gain_gold(&self, account: Account, account_user_id: &str, gain_golds: i32) -> Result<usize, diesel::result::Error>;
    async fn pay_gold(&self, account: Account, account_user_id: &str, gain_golds: i32) -> Result<usize, diesel::result::Error>;
}