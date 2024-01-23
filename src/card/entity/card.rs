use std::fmt;

pub struct Card {
    pub card_id: i32,              //카드 고유 id
    pub card_name: String,         //카드 이름 예시: 세라핌
    pub card_front_back: i32,      //카드 앞면 뒷면
    pub card_exist_status: i32,    // 카드 존재 상태 예시: 핸드 묘지 덱 필드 로스트존
    pub card_race: i32,       // 카드 종족 예시 언데드 휴면 천사                      //불변
    pub card_grade: i32,      // 카드 등급  전설 신화 등등                          // 불변
    pub card_type: i32,            //카드 종류 함정 마법 서포트 도구 등등             // 불변
    pub card_property: i32,   // 카드 속성 반신반인 성기사 등등                      //불변
    pub card_attack_point: i32,    // 공격력
    pub card_health_point: i32,    // 체력 HP
    pub card_activation_energy: i32, //필요 에너지
    pub card_current_tribe_energy: i32,  //현재 달고 있는 각 종족 기본 에너지 예: 천사 덱이면 신성력이 기본 에너지임 휴먼은 소울에너지 등등
    pub card_current_special_energy: i32,// 현재 달고 있는 스페셜 에너지 빙결에너지 화염 에너지 등등
    pub card_attack_type: i32,     // 물리 or 마법 공격 타입                                   //불변
    pub card_attack_distance: i32, // 카드 공격 거리 타입 원거리(근접) 근거리(직접)                 //불변
    pub action_count: i32,          //스킬이든 기본 공격이든 한턴에 한번만 되니까 이거 체크할 거 필요함
    pub card_passive1: i32,        // 카드 패시브 1                                          // 카드마다 가지고 있는효과는 다르고 턴이지나도 불변?
    pub card_passive2: i32,        // 패시브 2
    pub card_passive3: i32,        // 패시브 3
    pub card_skill1: i32,          // 스킬 1                                               //카드 마다 가지고 있는 스킬은 다르고 턴이 지나도 불변?
    pub card_skill2: i32,         //스킬 2
    pub card_skill3: i32,         //스킬 3 아 소요 에너지는 스킬에서 얼마 소모하면 현재 에너지에서 까면 됨

}