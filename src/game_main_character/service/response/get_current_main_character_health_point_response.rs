use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GetCurrentMainCharacterHealthPointResponse {
    current_health_point: i32,
}

impl GetCurrentMainCharacterHealthPointResponse {
    pub fn new(current_health_point: i32) -> Self {
        GetCurrentMainCharacterHealthPointResponse { current_health_point }
    }

    pub fn get_current_health_point(&self) -> i32 {
        self.current_health_point
    }
}