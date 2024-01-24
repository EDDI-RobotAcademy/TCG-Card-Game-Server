use async_trait::async_trait;

#[async_trait]
pub trait RedisInMemoryService {
    async fn save_key_and_value(&mut self, key: &str, value: &str);
    async fn get_value_with_key(&mut self, key: &str) -> Option<String>;
    async fn delete_value_with_key(&mut self, key: &str);
}