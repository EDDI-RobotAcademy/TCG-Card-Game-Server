#[derive(Debug)]
pub struct CheckedResponseRequest {
    address: String,
}

impl CheckedResponseRequest {
    pub fn new(address: &str) -> Self {
        CheckedResponseRequest {
            address: address.to_string(),
        }
    }

    pub fn get_address(&self) -> &str { &self.address }
}