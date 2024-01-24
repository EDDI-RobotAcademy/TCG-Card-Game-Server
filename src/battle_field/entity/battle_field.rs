#[derive(Debug)]
pub struct BattleField {
    count_turn: i32,
    environmental_card: bool,
}

impl BattleField {
    pub fn next_turn(&mut self) {
        self.count_turn += 1;

    }

    pub fn environmental_card(&self) -> bool { &self.environmental_card }


}