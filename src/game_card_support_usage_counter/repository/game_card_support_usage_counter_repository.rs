pub trait GameCardSupportUsageCounterRepository {
    fn create_support_card_usage_counter_object(&mut self, account_unique_id: i32) -> bool;
    fn reset_support_card_usage_counter(&mut self, account_unique_id: i32) -> bool;
    fn check_support_card_usage_counter(&mut self, account_unique_id: i32) -> i32;
    fn update_support_card_usage_counter(&mut self, account_unique_id: i32) -> bool;
}