#[derive(Debug,PartialEq)]
pub struct IsOpponentMulliganFinishedResponse {
    is_finished: bool
}

impl IsOpponentMulliganFinishedResponse {
    pub fn new(is_finished: bool) -> Self {
        IsOpponentMulliganFinishedResponse {
            is_finished
        }
    }

    pub fn is_finished(&self) -> bool {
        self.is_finished
    }
}