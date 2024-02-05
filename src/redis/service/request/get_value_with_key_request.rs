#[derive(Debug)]
pub struct GetValueWithKeyRequest {
    key: String,
}

impl GetValueWithKeyRequest {
    pub fn new(key: &str) -> Self {
        GetValueWithKeyRequest {
            key: key.to_string(),
        }
    }

    pub fn key(&self) -> &str{ &self.key }
}