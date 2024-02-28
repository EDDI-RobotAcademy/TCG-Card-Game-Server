#[derive(Debug)]
pub struct GenerateMySpecificUnitHealthPointDataRequest {
    my_unit_index: i32,
    my_unit_updated_health_point: i32,
}

impl GenerateMySpecificUnitHealthPointDataRequest {
    pub fn new(my_unit_index: i32,
               my_unit_updated_health_point: i32,) -> Self {
        GenerateMySpecificUnitHealthPointDataRequest {
            my_unit_index,
            my_unit_updated_health_point
        }
    }

    pub fn get_my_unit_index(&self) -> i32 { self.my_unit_index }

    pub fn get_my_unit_updated_health_point(&self) -> i32 { self.my_unit_updated_health_point }
}