use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AddDeadUnitListToTombResponse {
    is_success: bool,
}

impl AddDeadUnitListToTombResponse {
    pub fn new(is_success: bool) -> Self {
        AddDeadUnitListToTombResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}