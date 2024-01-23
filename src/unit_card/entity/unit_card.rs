pub struct Unitcard {
    pub card_id: i32,                   //카드 고유 id                                 불변
    pub card_name: String,              //카드 이름 예시: 세라핌                          불변
    pub card_race: i32,                 // 카드 종족 예시 언데드 휴면 천사                  불변
    pub card_grade: i32,                // 카드 등급  전설 신화 등등                       불변
    pub card_activation_energy: i32,       //기본 공격을 활성화 하기 위한 에너지
    pub unit_attack_point: i32,         // 공격력
    pub unit_health_point: i32,         // HP 피통
    pub card_passive1: i32,            // 카드 패시브 1                                          // 카드마다 가지고 있는효과는 다르고 턴이지나도 불변?
    pub card_passive2: i32,             // 패시브 2
    pub card_passive3: i32,             // 패시브 3
    pub card_skill1: i32,               // 스킬 1                                               //카드 마다 가지고 있는 스킬은 다르고 턴이 지나도 불변?
    pub card_skill2: i32,               //스킬 2
    pub card_skill3: i32,               //스킬 3 아 소요 에너지는 스킬에서 계산후   현재 에너지에서 까면 될란가

}