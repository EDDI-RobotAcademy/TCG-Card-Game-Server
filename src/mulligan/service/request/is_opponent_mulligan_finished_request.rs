#[derive(Debug)]
pub struct IsOpponentMulliganFinishedRequest {
    opponent_unique_id: i32,
}

impl IsOpponentMulliganFinishedRequest {
    pub fn new(
        opponent_unique_id: i32,
    ) -> Self {

        IsOpponentMulliganFinishedRequest {
            opponent_unique_id,
        }
    }

    pub fn get_opponent_unique_id(&self) -> i32 {
        self.opponent_unique_id
    }
}
