#[derive(Debug)]
pub struct GameTurn {
    turn: i32
}

impl GameTurn {
    pub fn new() -> Self {
        GameTurn { turn: 1 }
    }

    pub fn get_turn(&self) -> i32 {
        self.turn
    }

    pub fn next_turn(&mut self) {
        self.turn += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game_turn() {
        let game_turn = GameTurn::new();
        println!("Initial Turn: {}", game_turn.get_turn());
        assert_eq!(game_turn.get_turn(), 1);
    }

    #[test]
    fn test_next_turn() {
        let mut game_turn = GameTurn::new();
        game_turn.next_turn();
        println!("Current Turn: {}", game_turn.get_turn());
        assert_eq!(game_turn.get_turn(), 2);
    }
}
