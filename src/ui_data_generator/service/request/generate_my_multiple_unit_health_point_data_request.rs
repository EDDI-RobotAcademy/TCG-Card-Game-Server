#[derive(Debug)]
pub struct GenerateMyMultipleUnitHealthPointDataRequest {
    my_unit_health_point_tuple_list: Vec<(i32, i32)>,
}

impl GenerateMyMultipleUnitHealthPointDataRequest {
    pub fn new(my_unit_health_point_tuple_list: Vec<(i32, i32)>,) -> Self {
        GenerateMyMultipleUnitHealthPointDataRequest {
            my_unit_health_point_tuple_list,
        }
    }

    pub fn get_my_unit_health_point_tuple_list(&self) -> Vec<(i32, i32)> {
        self.my_unit_health_point_tuple_list.clone()
    }
}