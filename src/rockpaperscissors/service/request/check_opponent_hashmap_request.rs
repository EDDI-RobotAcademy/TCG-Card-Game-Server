#[derive(Debug)]
pub struct CheckOpponentHashmapRequest {
    opponent_id:i32,

}

impl CheckOpponentHashmapRequest {
    pub fn new(opponent_id:i32) -> Self {
        CheckOpponentHashmapRequest {

            opponent_id,

        }
    }

    pub fn get_opponent_id(&self) -> i32 {
        self.opponent_id
    }


}
