#[derive(Debug)]
pub struct SaveKeyAndValueRequest {
    key: String,
    value: String,
}

impl SaveKeyAndValueRequest {
    pub fn new(key: &str, value: &str) -> Self {
        SaveKeyAndValueRequest {
            key: key.to_string(),
            value: value.to_string(),
        }
    }

    pub fn key(&self) -> &str{ &self.key }

    pub fn value(&self) -> &str { &self.value }
}