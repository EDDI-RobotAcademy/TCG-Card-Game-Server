#[derive(Debug)]
pub struct CheckConnectingStatusRequest {
    address: String,
}

impl CheckConnectingStatusRequest {
    pub fn new(address: &str) -> Self {
        CheckConnectingStatusRequest {
            address: address.to_string(),
        }
    }

    pub fn get_address(&self) -> &str { &self.address }
}