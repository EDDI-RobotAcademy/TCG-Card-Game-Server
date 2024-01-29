use rand::prelude::SliceRandom;
use std::fmt;
pub(crate) fn request_shuffle_deck_for_test2(deck_list_tuples: &Vec<(Unitcard, i32)>) -> Vec<Unitcard> {
    let mut list_iter: Vec<Unitcard> = Vec::new();

    for (unit_card, card_count) in deck_list_tuples {
        for _ in 0..*card_count {
            list_iter.push(unit_card.clone());
        }
    }

    list_iter.shuffle(&mut rand::thread_rng());

    list_iter
}
//TODO 0번쨰 1번째 2번쨰 0번쨰 부터 시작합니다 덱 카드는 일단 한마리 소환 가능
pub(crate) fn summon_one_unit_card(unit_battlefield: &mut Vec<Unitcard>, hand_cards: &mut Vec<Unitcard>, hand_cards_number: i32){
    if (0..hand_cards.len() as i32).contains(&hand_cards_number)
    {
        let summoned_card = hand_cards.remove(hand_cards_number as usize);
        println!("핸드에서 {:?}번째 카드 소환 ->{:?}",hand_cards_number, summoned_card.card_id);
        unit_battlefield.push(summoned_card);
    }


}
pub(crate) fn draw_cards2(deck: &mut Vec<Unitcard>, number_to_take: usize) -> Vec<Unitcard> {
    deck.drain(..number_to_take).collect()
}
pub(crate) fn search_card_from_deck_effect2(deck: &mut Vec<Unitcard>, hand_cards: &mut Vec<Unitcard>, count:i32) -> Vec<Unitcard> {
    let mut remain_deck = Vec::new();

    // 추출한 값의 개수를 카운트하는 변수
    let mut card_count = 0;

    //TODO 일단 count는 서치에서 뽑을 카드 개수인데 예를 들어 보기가 0 0 0 0 0 이렇게 있으면 사용자가 3번쨰 5번쨰 고를 수 있으니까 이거 나중에 수정하기
    while let Some(index) = deck.iter().position(|x| x.card_grade>=4 && x.card_grade<=5) {
        let card = deck.drain(index..=index).next();
        if let Some(card) = card {
            hand_cards.push(card);
            card_count += 1;
        }

        // 추출한 값의 개수가 count에 도달하면 루프 종료
        if card_count == count {
            break;
        }
    }

    // 남은 deck의 값을 remaining_deck에 추가
    remain_deck.extend(deck.drain(..));

    remain_deck
}

#[derive(Debug, Clone)] // Debug 및 Clone 트레이트를 자동으로 구현
pub struct Unitcard {
    pub card_id: i32,
    pub card_name: String,
    pub card_race: String,
    pub card_kinds: String,
    pub card_grade: i32,
    pub attack_point: i32,
    pub health_point: i32,
    pub activation_energy: i32,
    pub skill: Vec<i32>,
    pub passive: Vec<i32>,

}
#[derive(Debug)]
enum Cardgrade {
    general=1,
    uncommon,
    hero,
    legend,
    myth,
}


#[cfg(test)]
mod tests {
    use rand::prelude::SliceRandom;
    use serde::de::Unexpected::Unit;
    use crate::ugly_tests::draw_deck::test_draw_five_from_deck::draw_cards;
    use crate::ugly_tests::search_card_in_deck::test_search_card_from_deck::search_card_from_deck_effect;

    use super::{draw_cards2, request_shuffle_deck_for_test2, search_card_from_deck_effect2, summon_one_unit_card, Unitcard}; // UnitCard 구조체 가져오기

    #[test]
    fn test_summon_unit() {

        let mut unit_battlefield: Vec<Unitcard> = Vec::new();
        let mut unit_card_encyclopedia:Vec<Unitcard> = Vec::new();
        unit_card_encyclopedia.push(Unitcard {
            card_id: 0,
            card_name: String::from("세라핌"),
            card_race: String::from("천사족"),
            card_kinds: String::from("유닛"),
            card_grade: 5,
            attack_point: 10,
            health_point: 20,
            activation_energy: 5,
            skill: vec![1, 2, 3],
            passive: vec![4, 5, 6],
        });
        unit_card_encyclopedia.push(Unitcard {
            card_id: 1,
            card_name: String::from("마검 블레이드"),
            card_race: String::from("언데드족"),
            card_kinds: String::from("유닛"),
            card_grade: 4,
            attack_point: 15,
            health_point: 25,
            activation_energy: 7,
            skill: vec![7, 8, 9],
            passive: vec![10, 11, 12],
        });
        unit_card_encyclopedia.push(Unitcard {
            card_id: 2,
            card_name: String::from("유닛2"),
            card_race: String::from("천사족"),
            card_kinds: String::from("유닛"),
            card_grade: 1,
            attack_point: 10,
            health_point: 20,
            activation_energy: 5,
            skill: vec![1, 2, 3],
            passive: vec![4, 5, 6],
        });
        unit_card_encyclopedia.push(Unitcard {
            card_id: 3,
            card_name: String::from("유닛3"),
            card_race: String::from("천사족"),
            card_kinds: String::from("유닛"),
            card_grade: 2,
            attack_point: 10,
            health_point: 20,
            activation_energy: 5,
            skill: vec![1, 2, 3],
            passive: vec![4, 5, 6],
        });
        unit_card_encyclopedia.push(Unitcard {
            card_id: 4,
            card_name: String::from("유닛4"),
            card_race: String::from("천사족"),
            card_kinds: String::from("유닛"),
            card_grade: 3,
            attack_point: 10,
            health_point: 20,
            activation_energy: 5,
            skill: vec![1, 2, 3],
            passive: vec![4, 5, 6],
        });
        unit_card_encyclopedia.push(Unitcard {
            card_id: 5,
            card_name: String::from("유닛5"),
            card_race: String::from("천사족"),
            card_kinds: String::from("유닛"),
            card_grade: 1,
            attack_point: 10,
            health_point: 20,
            activation_energy: 5,
            skill: vec![1, 2, 3],
            passive: vec![4, 5, 6],
        });
        unit_card_encyclopedia.push(Unitcard {
            card_id: 6,
            card_name: String::from("유닛6"),
            card_race: String::from("천사족"),
            card_kinds: String::from("유닛"),
            card_grade: 2,
            attack_point: 10,
            health_point: 20,
            activation_energy: 5,
            skill: vec![1, 2, 3],
            passive: vec![4, 5, 6],
        });
        let deck_list_tuples: Vec<(Unitcard, i32)> = vec![
            (unit_card_encyclopedia[0].clone(), 2),
            (unit_card_encyclopedia[1].clone(), 2),
            (unit_card_encyclopedia[2].clone(), 1),
            (unit_card_encyclopedia[3].clone(), 1),
            (unit_card_encyclopedia[4].clone(), 1),
            (unit_card_encyclopedia[5].clone(), 1),
            (unit_card_encyclopedia[6].clone(), 1),
        ];
        let mut player_deck_list=request_shuffle_deck_for_test2(&deck_list_tuples);
        for card in &player_deck_list {
            println!("덱 출력 -> Card_id: {:?}", card.card_id);
        }
        let mut number_to_take=5;
        let mut hand_cards: Vec<Unitcard> = Vec::new();
        hand_cards= draw_cards2(&mut player_deck_list,number_to_take);
        for card in &hand_cards {
            println!("손패 출력 -> Card_id: {:?}", card.card_id);
        }
        for card in &player_deck_list {
            println!("남은 덱 -> Card_id: {:?}", card.card_id);
        }
        player_deck_list=search_card_from_deck_effect2(&mut player_deck_list,&mut hand_cards,1);
        for card in &player_deck_list {
            println!("서치 후 남은 덱 -> Card_id: {:?}", card.card_id);
        }
        for card in &hand_cards {
            println!("서치 후 손패 출력 -> Card_id: {:?}", card.card_id);
        }


        // summon_one_unit_card(&mut unit_battlefield, &mut hand_cards, 3);
        // for card in &unit_battlefield {
        //     println!("배틀필드 출력 ->  {:?}", unit_battlefield[0].card_id);
        // }
        // for card in &hand_cards {
        //     println!("소환후 손패 출력 -> Card_id: {:?}", card.card_id);
        // }
        // for card in &player_deck_list {
        //     println!("소환후 남은 덱 -> Card_id: {:?}", card.card_id);
        // }




    }
}