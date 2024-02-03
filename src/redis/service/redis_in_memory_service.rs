use async_trait::async_trait;
use crate::redis::service::request::save_daily_key_and_value_request::SaveDailyKeyAndValueRequest;
use crate::redis::service::response::save_daily_key_and_value_response::SaveDailyKeyAndValueResponse;

#[async_trait]
pub trait RedisInMemoryService {
    async fn save_key_and_value(&mut self, key: &str, value: &str);
    async fn get_value_with_key(&mut self, key: &str) -> Option<String>;
    async fn delete_value_with_key(&mut self, key: &str);
    async fn save_daily_key_and_value(&self, save_daily_key_and_value_request: SaveDailyKeyAndValueRequest) -> SaveDailyKeyAndValueResponse;
}