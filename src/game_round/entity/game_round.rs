#[derive(Debug)]
pub struct GameRound {
    round: i32
}

impl GameRound {
    pub fn new() -> Self {
        GameRound { round: 1 }
    }

    pub fn get_round(&self) -> i32 {
        self.round
    }

    pub fn next_round(&mut self) {
        self.round += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game_round() {
        let game_round = GameRound::new();
        println!("Initial Turn: {}", game_round.get_round());
        assert_eq!(game_round.get_round(), 1);
    }

    #[test]
    fn test_next_round() {
        let mut game_round = GameRound::new();
        game_round.next_round();
        println!("Current Round: {}", game_round.get_round());
        assert_eq!(game_round.get_round(), 2);
    }
}
