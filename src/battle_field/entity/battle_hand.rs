#[derive(Debug)]
pub struct BattleHand {
    hand_list: Vec<i32>,


}

impl BattleHand {
    pub fn new() -> BattleHand {
        BattleHand {
            hand_list: Vec::new(),
        }
    }
    pub fn use_card(&mut self, num: i32) -> i32 {
        self.hand_list.remove(num)

    }
}