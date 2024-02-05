use async_trait::async_trait;

#[async_trait]
pub trait RedisInMemoryRepository {
    async fn set_permanent(&mut self, key: &str, value: &str);
    async fn set_with_expired_time(&mut self, key: &str, value: &str, expiry_seconds: Option<u32>);
    async fn get(&mut self, key: &str) -> Option<String>;
    async fn del(&mut self, key: &str);
    async fn set_with_expired_target_time(&mut self, key: &str, value: &str);
}