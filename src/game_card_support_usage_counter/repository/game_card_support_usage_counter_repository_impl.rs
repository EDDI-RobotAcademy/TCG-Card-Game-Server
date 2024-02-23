use std::collections::HashMap;
use std::sync::Arc;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::game_card_support_usage_counter::entity::support_card_usage_counter::SupportCardUsageCounter;
use crate::game_card_support_usage_counter::repository::game_card_support_usage_counter_repository::GameCardSupportUsageCounterRepository;

pub struct GameCardSupportUsageCounterRepositoryImpl {
    support_card_usage_counter_map: HashMap<i32, SupportCardUsageCounter>
}

impl GameCardSupportUsageCounterRepositoryImpl {
    pub fn new() -> Self {
        GameCardSupportUsageCounterRepositoryImpl {
            support_card_usage_counter_map: HashMap::new()
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<GameCardSupportUsageCounterRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<GameCardSupportUsageCounterRepositoryImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        GameCardSupportUsageCounterRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

impl GameCardSupportUsageCounterRepository for GameCardSupportUsageCounterRepositoryImpl {
    fn create_support_card_usage_counter_object(&mut self, account_unique_id: i32) -> bool {
        println!("GameCardSupportUsageCounterRepositoryImpl: create_game_support_card_usage_counter_object()");

        let new_support_card_usage_counter = SupportCardUsageCounter::new();
        self.support_card_usage_counter_map.insert(account_unique_id, new_support_card_usage_counter);

        true
    }

    fn reset_support_card_usage_counter(&mut self, account_unique_id: i32) -> bool {
        println!("GameCardSupportUsageCounterRepositoryImpl: reset_support_card_usage_counter()");

        if let Some(support_card_usage_counter) =
            self.support_card_usage_counter_map.get_mut(&account_unique_id) {
            support_card_usage_counter.set_count(0);
        }

        true
    }

    fn check_support_card_usage_counter(&mut self, account_unique_id: i32) -> i32 {
        println!("GameCardSupportUsageCounterRepositoryImpl: check_support_card_usage_counter()");

        if let Some(support_card_usage_counter) =
            self.support_card_usage_counter_map.get_mut(&account_unique_id) {
            return support_card_usage_counter.get_count()
        }

        -1
    }

    fn update_support_card_usage_counter(&mut self, account_unique_id: i32) -> bool {
        println!("GameCardSupportUsageCounterRepositoryImpl: update_support_card_usage_counter()");

        if let Some(support_card_usage_counter) =
            self.support_card_usage_counter_map.get_mut(&account_unique_id) {
            support_card_usage_counter.add_count();
        }

        true
    }

    fn remove_game_support_card_usage_counter_hash_by_account_unique_id(&mut self, account_unique_id: i32) -> bool {
        if let Some(support_card_usage_counter) = self.support_card_usage_counter_map.get_mut(&account_unique_id) {
            self.support_card_usage_counter_map.remove(&account_unique_id);
            return true
        }
        return false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_all_features() {
        let repository_mutex =
            GameCardSupportUsageCounterRepositoryImpl::get_instance();

        let mut guard = repository_mutex.lock().await;
        let account_unique_id = 1;

        guard.create_support_card_usage_counter_object(account_unique_id);

        guard.update_support_card_usage_counter(account_unique_id);

        let result = guard.check_support_card_usage_counter(account_unique_id);
        assert_eq!(result, 1);

        guard.reset_support_card_usage_counter(account_unique_id);
        assert_eq!(guard.support_card_usage_counter_map.get(&account_unique_id),
                   Some(SupportCardUsageCounter::new()).as_ref())
    }
}