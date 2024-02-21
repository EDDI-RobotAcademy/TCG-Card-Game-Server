#[derive(Debug)]
pub struct UpdateAccountCardDbRequest {
    account_unique_id: i32,
    update_card_list: Vec<i32>
}
impl UpdateAccountCardDbRequest {
    pub fn new(account_unique_id: i32, update_card_list: Vec<i32>) -> Self {
        UpdateAccountCardDbRequest {
            account_unique_id,
            update_card_list
        }
    }

    pub fn account_unique_id(&self) -> i32 { self.account_unique_id }
    pub fn update_card_list(&self) -> &Vec<i32> { &self.update_card_list }

}