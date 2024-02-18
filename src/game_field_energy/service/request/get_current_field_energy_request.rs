#[derive(Debug)]
pub struct GetCurrentFieldEnergyRequest {
    account_unique_id: i32,
}

impl GetCurrentFieldEnergyRequest {
    pub fn new(account_unique_id: i32) -> Self {
        GetCurrentFieldEnergyRequest {
            account_unique_id,
        }
    }
    pub fn get_account_unique_id(&self) -> i32 { self.account_unique_id }
}