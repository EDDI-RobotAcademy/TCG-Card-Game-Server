#[derive(Debug)]
pub struct RecordMulliganFinishRequest {
    account_unique_id: i32,
}

impl RecordMulliganFinishRequest {
    pub fn new(
        account_unique_id: i32,
    ) -> Self {

        RecordMulliganFinishRequest {
            account_unique_id,
        }
    }

    pub fn get_account_unique_id(&self) -> i32 {
        self.account_unique_id
    }
}
