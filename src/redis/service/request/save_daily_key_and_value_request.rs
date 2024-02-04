#[derive(Debug)]
pub struct SaveDailyKeyAndValueRequest {
    key: String,
    value: String,
}

impl SaveDailyKeyAndValueRequest {
    pub fn new(key: &str, value: &str) -> Self {
        SaveDailyKeyAndValueRequest {
            key: key.to_string(),
            value: value.to_string(),
        }
    }

    pub fn key(&self) -> &str{ &self.key }

    pub fn value(&self) -> &str { &self.value }
}