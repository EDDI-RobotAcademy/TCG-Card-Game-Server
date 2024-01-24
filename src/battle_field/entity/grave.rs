#[derive(Debug)]
pub struct Grave {
    grave_list: Vec<i32>,

}

impl Grave {
    pub fn new() -> Grave {
        Grave {
            grave_list: Vec::new(),
        }
    }
    pub fn go_grave (&mut self, card_num: i32) {
        self.grave_list.push(card_num);
    }
}