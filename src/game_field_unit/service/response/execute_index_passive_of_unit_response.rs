use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecuteIndexPassiveOfUnitResponse {
    is_success: bool
}

impl ExecuteIndexPassiveOfUnitResponse {
    pub fn new(is_success: bool) -> Self {
        ExecuteIndexPassiveOfUnitResponse { is_success }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}