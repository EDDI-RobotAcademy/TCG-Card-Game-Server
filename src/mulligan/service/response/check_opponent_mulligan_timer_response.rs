#[derive(Debug,PartialEq)]
pub struct CheckOpponentMulliganTimerResponse {
    is_running: bool
}

impl CheckOpponentMulliganTimerResponse {
    pub fn new(is_running: bool) -> Self {
        CheckOpponentMulliganTimerResponse {
            is_running
        }
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }
}