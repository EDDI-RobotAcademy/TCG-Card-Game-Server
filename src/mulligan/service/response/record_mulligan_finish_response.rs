#[derive(Debug, PartialEq)]
pub struct RecordMulliganFinishResponse {
    is_success: bool
}

impl RecordMulliganFinishResponse {
    pub fn new(is_success: bool) -> Self {
        RecordMulliganFinishResponse {
            is_success
        }
    }

    pub fn is_success(&self) -> bool {
        self.is_success
    }
}