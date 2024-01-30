#[derive(Debug)]
pub struct BattleCard {
    account_id: i32,
    card_id: i32,
    name: String,
    race: i32,
    grade: i32,
    kind: i32,
    job: i32,
    activation_energy: i32,
    attack_point: i32,
    health_point: i32,
    passive1: i32,
    passive2: i32,
    passive3: i32,
    skill1: i32,
    skill2: i32,
    skill3: i32,
    effect1: i32,
    effect2: i32,
    effect3: i32,
    current_location: i32,
    current_energy: i32,
}

impl BattleCard {
    pub fn new(account_id: i32) -> Self {
        BattleCard {
            account_id,
            card_id: 0,
            name: "".to_string(),
            race: 0,
            grade: 0,
            kind: 0,
            job: 0,
            activation_energy: 0,
            attack_point: 0,
            health_point: 0,
            passive1: 0,
            passive2: 0,
            passive3: 0,
            skill1: 0,
            skill2: 0,
            skill3: 0,
            effect1: 0,
            effect2: 0,
            effect3: 0,
            current_location: 1,
            current_energy: 0,
        }
    }
    pub fn set_card_id(&mut self, card_id_i32: i32) {
        self.card_id = card_id_i32
    }
    pub fn set_card_name(&mut self, card_name_string: &str) {
        self.name = card_name_string.to_string()
    }
    pub fn set_card_race(&mut self, card_race_string: &str) {
        self.race = card_race_string.parse().expect("Not an integer value")
    }
    pub fn set_card_grade(&mut self, card_grade_string: &str) {
        self.grade = card_grade_string.parse().expect("Not an integer value")
    }
    pub fn set_card_kind(&mut self, card_kind_string: &str) {
        self.kind = card_kind_string.parse().expect("Not an integer value")
    }
    pub fn set_card_activation_energy(&mut self, card_activation_energy_string: &str) {
        self.activation_energy = card_activation_energy_string.parse().expect("Not an integer value")
    }
    pub fn set_card_attack_point(&mut self, card_attack_point_string: &str) {
        self.attack_point = card_attack_point_string.parse().expect("Not an integer value")
    }
    pub fn set_card_health_point(&mut self, card_health_point_string: &str) {
        self.health_point = card_health_point_string.parse().expect("Not an integer value")
    }
}