use std::fmt;

pub struct Effectcard{

    pub effect_card_type: i32, //  0이면 도구 1이면 함정 2면 서포트 3이면 환경 등등 효과 카드 중 진짜 카드 타입
    pub consume_energy: i32,  //소모하는 에너지
    pub effect:i32,             //effect_card 효과들을 구현해놓고 번호 매겨서 effect가 0라면 0번 효과 가지고 있다?

}