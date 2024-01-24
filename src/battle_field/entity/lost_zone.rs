#[derive(Debug)]
pub struct LostZone {
    lost_zone: Vec<i32>,

}

impl LostZone {
    pub fn new() -> LostZone {
        LostZone {
            lost_zone: Vec::new(),
        }
    }
    pub fn go_lost_zone (&mut self, card_num: i32) {
        self.lost_zone.push(card_num);
    }
}