use std::fmt;

pub struct Energycard {
    pub card_id: i32,                   //카드 고유 id                                 불변
    pub card_name: String,              //카드 이름 예시: 세라핌                          불변
    pub card_race: i32,                 // 카드 종족 예시 언데드 휴면 천사                  불변
    pub card_grade: i32,                // 카드 등급  전설 신화 등등                       불변

}