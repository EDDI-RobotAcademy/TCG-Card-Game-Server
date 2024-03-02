use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCurrentHealthPointOfAllFieldUnitResponse {
    current_unit_health_point_list: Vec<(i32, i32)>,
}

impl GetCurrentHealthPointOfAllFieldUnitResponse {
    pub fn new(current_unit_health_point_list: Vec<(i32, i32)>,) -> Self {
        GetCurrentHealthPointOfAllFieldUnitResponse {
            current_unit_health_point_list
        }
    }
    pub fn get_current_unit_health_point(&self) -> &Vec<(i32, i32)> { &self.current_unit_health_point_list }
}