use std::fmt;

pub struct Card {
    pub card_id: i32,                      // 카드의 고유 번호
    pub card_name: String,                 // 세라핌 등 카드 이름
    pub card_info: i32,                    //등급 전설 종족 천사 타입 아이템
}