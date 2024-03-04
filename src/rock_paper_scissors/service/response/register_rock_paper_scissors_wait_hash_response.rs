#[derive(Debug)]
pub struct RegisterRockPaperScissorsWaitHashResponse {
    is_success: bool,
}

impl RegisterRockPaperScissorsWaitHashResponse {
    pub fn new(is_success: bool) -> Self {
        RegisterRockPaperScissorsWaitHashResponse {
            is_success
        }
    }

    pub fn get_is_success(&self) -> bool {
        self.is_success
    }
}
