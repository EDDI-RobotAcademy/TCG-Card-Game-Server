use std::fmt;

pub struct Tokencard {
    pub card_id: i32,                   //카드 고유 id                                 불변
    pub card_name: String,              //카드 이름 예시: 세라핌                          불변
    pub card_race: i32,                 // 카드 종족 예시 언데드 휴면 천사                  불변
    pub unit_attack_point: i32,         // 공격력
    pub unit_health_point: i32          // HP 피통
}