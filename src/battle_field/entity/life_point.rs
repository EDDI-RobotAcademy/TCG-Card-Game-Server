#[derive(Debug)]
pub struct LifePoint {
    life_point: i32,

}

impl LifePoint {
    pub fn init_lifepoint (&mut self) {
        self.life_point = 100;
    }


}