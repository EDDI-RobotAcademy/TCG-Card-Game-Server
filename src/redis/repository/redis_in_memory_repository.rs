use async_trait::async_trait;

#[async_trait]
pub trait RedisInMemoryRepository {
    async fn set(&mut self, key: &str, value: &str);
    async fn get(&mut self, key: &str) -> Option<String>;
}