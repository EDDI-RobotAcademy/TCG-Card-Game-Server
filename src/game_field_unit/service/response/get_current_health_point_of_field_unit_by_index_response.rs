use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCurrentHealthPointOfFieldUnitByIndexResponse {
    current_unit_health_point: i32,
}

impl GetCurrentHealthPointOfFieldUnitByIndexResponse {
    pub fn new(current_unit_health_point: i32) -> Self {
        GetCurrentHealthPointOfFieldUnitByIndexResponse {
            current_unit_health_point
        }
    }
    pub fn get_current_unit_health_point(&self) -> i32 { self.current_unit_health_point }
}