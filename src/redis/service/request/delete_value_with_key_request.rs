#[derive(Debug)]
pub struct DeleteValueWithKeyRequest {
    key: String,
}

impl DeleteValueWithKeyRequest {
    pub fn new(key: &str) -> Self {
        DeleteValueWithKeyRequest {
            key: key.to_string(),
        }
    }

    pub fn key(&self) -> &str{ &self.key }
}