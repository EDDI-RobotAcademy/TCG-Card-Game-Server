use std::fmt;

pub struct Cardtype {

    pub unit_card: i32,
    pub energy_card: i32,
    pub effect_card: i32,                  //무조건 세개 중 하나만 1이어야한다다 유닛카드라면 1 00 에너지 카드라면 010 등 표현 방식 미정

}