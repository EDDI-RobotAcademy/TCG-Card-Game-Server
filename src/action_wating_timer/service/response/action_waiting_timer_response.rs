#[derive(Debug)]
pub struct ActionWaitingTimerResponse {
    turn_over: bool,
}

impl ActionWaitingTimerResponse {
    pub fn new(turn_over: bool) -> Self {
        ActionWaitingTimerResponse {
            turn_over,
        }
    }
}