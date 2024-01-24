use std::fmt;

pub struct Unitcard {
    pub unit_job: i32,              //성기사 반신반인 등 유닛 속성
    pub health_point: i32,          // HP
    pub attack_point: i32,          // 공격력
    pub attack_type:i32,            // 물리 공격이나 마법 공격 이냐
    pub attack_range: i32,          // 근접(단거리) 공격 이냐 간접(원거리)공격이냐
    pub activation_energy: i32,     // 기본 공격이나 스킬 등 액션에 대해 필요한 에너지
    pub unit_skill1 :i32,           // 한 유닛이 최대 스킬 3개 보유 가능이고 스킬이 하나만 있다면 나머지는 0이나 none 처리?
    pub unit_skill2 :i32,
    pub unit_skill3 :i32,
    pub unit_passive1: i32,        // 한 유닛이 최대 패시브 3개 보유 가능이고 패시브가 하나만 있다면 나머지는 0이나 none 처리?
    pub unit_passive2: i32,
    pub unit_passive3: i32,

}

//패시브,스킬들을 다 구현해놓고 패시브,스킬들의 고유번호가 있따면 unit_skill1 이 3이라면 3번 스킬을 가지고 있다?