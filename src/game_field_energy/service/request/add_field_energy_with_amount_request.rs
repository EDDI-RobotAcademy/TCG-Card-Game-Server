#[derive(Debug)]
pub struct AddFieldEnergyWithAmountRequest {
    account_unique_id: i32,
    amount_to_add: i32,
}

impl AddFieldEnergyWithAmountRequest {
    pub fn new(account_unique_id: i32, amount_to_add: i32) -> Self {
        AddFieldEnergyWithAmountRequest {
            account_unique_id,
            amount_to_add,
        }
    }
    pub fn get_account_unique_id(&self) -> i32 { self.account_unique_id }
    pub fn get_amount_to_add(&self) -> i32 { self.amount_to_add }
}