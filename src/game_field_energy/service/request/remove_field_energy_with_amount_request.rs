#[derive(Debug)]
pub struct RemoveFieldEnergyWithAmountRequest {
    account_unique_id: i32,
    amount_to_remove: i32,
}

impl RemoveFieldEnergyWithAmountRequest {
    pub fn new(account_unique_id: i32, amount_to_remove: i32) -> Self {
        RemoveFieldEnergyWithAmountRequest {
            account_unique_id,
            amount_to_remove,
        }
    }
    pub fn get_account_unique_id(&self) -> i32 { self.account_unique_id }
    pub fn get_amount_to_remove(&self) -> i32 { self.amount_to_remove }
}