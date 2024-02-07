use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PlaceToTombResponse {
    is_success: bool,
}

impl PlaceToTombResponse {
    pub fn new(is_success: bool) -> Self {
        PlaceToTombResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}
