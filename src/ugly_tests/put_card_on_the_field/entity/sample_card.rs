#[derive(Debug)]
pub struct SampleCard {
    card_location: i32,
    card_race: i32,
    card_grade: i32,
    card_type: i32,
    activation_energy: i32,
    attack_point: i32,
    health_point: i32,
}

impl SampleCard {
    pub fn new(card_race: i32, card_grade: i32, card_type: i32,
               activation_energy: i32, attack_point: i32, health_point: i32) -> Self {
        SampleCard {
            card_location: 1, // 일단 핸드로 강제 캐스팅
            card_race,
            card_grade,
            card_type,
            activation_energy,
            attack_point,
            health_point
        }
    }
    pub fn card_location(&self) -> i32 { self.card_location }
    pub fn card_race(&self) -> i32 { self.card_race }
    pub fn card_grade(&self) -> i32 { self.card_grade }
    pub fn card_type(&self) -> i32 { self.card_type }
    pub fn activation_energy(&self) -> i32 { self.activation_energy }
    pub fn attack_point(&self) -> i32 { self.attack_point }
    pub fn health_point(&self) -> i32 { self.health_point }
}