#[derive(Debug)]
pub struct CheckOpponentChoiceRequest {
    opponent_unique_id: i32,

}

impl CheckOpponentChoiceRequest {
    pub fn new(opponent_unique_id: i32) -> Self {
        CheckOpponentChoiceRequest {
            opponent_unique_id,
        }
    }

    pub fn get_opponent_unique_id(&self) -> i32 {
        self.opponent_unique_id
    }
}
