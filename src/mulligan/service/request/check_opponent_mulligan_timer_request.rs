#[derive(Debug)]
pub struct CheckOpponentMulliganTimerRequest {
    opponent_unique_id: i32,
}

impl CheckOpponentMulliganTimerRequest {
    pub fn new(
        opponent_unique_id: i32,
    ) -> Self {

        CheckOpponentMulliganTimerRequest {
            opponent_unique_id,
        }
    }

    pub fn get_opponent_unique_id(&self) -> i32 {
        self.opponent_unique_id
    }
}
