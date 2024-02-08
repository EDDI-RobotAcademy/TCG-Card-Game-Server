use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddUnitToGameFieldResponse {
    is_success: bool
}

impl AddUnitToGameFieldResponse {
    pub fn new(is_success: bool) -> Self {
        AddUnitToGameFieldResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}
