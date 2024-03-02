#[derive(Debug)]
pub struct GenerateMyMultipleUnitDeathDataRequest {
    my_dead_unit_index_list: Vec<i32>,
}

impl GenerateMyMultipleUnitDeathDataRequest {
    pub fn new(my_dead_unit_index_list: Vec<i32>,) -> Self {
        GenerateMyMultipleUnitDeathDataRequest {
            my_dead_unit_index_list,
        }
    }

    pub fn get_my_dead_unit_index_list(&self) -> &Vec<i32> { &self.my_dead_unit_index_list }
}