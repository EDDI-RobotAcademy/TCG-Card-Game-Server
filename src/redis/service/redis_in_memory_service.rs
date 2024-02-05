use async_trait::async_trait;
use crate::redis::service::request::delete_value_with_key_request::DeleteValueWithKeyRequest;
use crate::redis::service::request::get_value_with_key_request::GetValueWithKeyRequest;
use crate::redis::service::request::save_daily_key_and_value_request::SaveDailyKeyAndValueRequest;
use crate::redis::service::request::save_key_and_value_request::SaveKeyAndValueRequest;
use crate::redis::service::response::delete_value_with_key_response::DeleteValueWithKeyResponse;
use crate::redis::service::response::get_value_with_key_response::GetValueWithKeyResponse;
use crate::redis::service::response::save_daily_key_and_value_response::SaveDailyKeyAndValueResponse;
use crate::redis::service::response::save_key_and_value_response::SaveKeyAndValueResponse;

#[async_trait]
pub trait RedisInMemoryService {
    async fn save_key_and_value(&self, save_key_and_value_request: SaveKeyAndValueRequest) -> SaveKeyAndValueResponse;
    async fn get_value_with_key(&self, get_value_with_key_request: GetValueWithKeyRequest) -> GetValueWithKeyResponse;
    async fn delete_value_with_key(&self, delete_value_with_key_request: DeleteValueWithKeyRequest) -> DeleteValueWithKeyResponse;
    async fn save_daily_key_and_value(&self, save_daily_key_and_value_request: SaveDailyKeyAndValueRequest) -> SaveDailyKeyAndValueResponse;
}