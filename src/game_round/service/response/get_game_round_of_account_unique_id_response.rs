#[derive(Debug, Clone)]
pub struct GetGameRoundOfAccountUniqueIdResponse {
    game_round: i32,
}

impl GetGameRoundOfAccountUniqueIdResponse {
    pub fn new(game_round: i32) -> Self {
        GetGameRoundOfAccountUniqueIdResponse { game_round }
    }

    pub fn get_game_round(&self) -> i32 {
        self.game_round
    }
}