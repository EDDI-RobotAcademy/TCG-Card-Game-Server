#[derive(Debug)]
pub struct CheckDrawChoiceResponse {
    is_success: bool,
}

impl CheckDrawChoiceResponse {
    pub fn new(is_success: bool) -> Self {
        CheckDrawChoiceResponse {
            is_success
        }
    }

    pub fn get_is_success(&self) -> bool {
        self.is_success
    }
}
