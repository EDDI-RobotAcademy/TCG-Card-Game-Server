#[derive(Debug,PartialEq)]
pub struct CheckOpponentHashmapResponse {
    opponent_check: bool

}

impl CheckOpponentHashmapResponse {
    pub fn new(opponent_check: bool) -> Self {
        CheckOpponentHashmapResponse {
            opponent_check

        }
    }

    pub fn get_opponent_check(&self) -> bool {
        self.opponent_check
    }


}
