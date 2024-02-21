#[derive(Debug)]
pub struct AddDeadUnitListToTombRequest {
    account_unique_id: i32,
    dead_unit_list: Vec<i32>
}

impl AddDeadUnitListToTombRequest {
    pub fn new(account_unique_id: i32, dead_unit_list: Vec<i32>) -> Self {
        AddDeadUnitListToTombRequest {
            account_unique_id,
            dead_unit_list
        }
    }

    pub fn get_account_unique_id(&self) -> i32 {
        self.account_unique_id
    }

    pub fn get_dead_unit_list(&self) -> &Vec<i32> {
        &self.dead_unit_list
    }
}