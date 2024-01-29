use rand::prelude::SliceRandom;
pub(crate) fn search_card_from_deck_effect(deck: &mut Vec<i32>, hand_cards: &mut Vec<i32>, value: i32, count: usize) -> Vec<i32> {
    let mut remain_deck = Vec::new();

    // 추출한 값의 개수를 카운트하는 변수
    let mut card_count = 0;

    // deck에서 value 값을 가진 요소를 추출하고 hand_cards에 추가
    while let Some(index) = deck.iter().position(|&x| x == value) {
        let card_value = deck.drain(index..=index).next();
        if let Some(value) = card_value {
            hand_cards.push(value);
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
//TODO 현재 테스트 코드에서는 벡터 안의 값을 검색해서 찾아왔지만 영웅 이하의 등급에서 몇장 가져온다 등등 아마 카드 등급이 범위이므로
//TODO 카드의 등급 및 덱을 나타내는 방식 수정  확정후 그에 맞는 데이터 타입 수정 필요
#[cfg(test)]
mod tests {
    use rand::prelude::SliceRandom;
    use crate::ugly_tests::draw_deck::test_draw_five_from_deck::draw_cards;
    use crate::ugly_tests::shuffle_deck::test_shuffle_deck::request_shuffle_deck_for_test;
    use crate::ugly_tests::search_card_in_deck::test_search_card_from_deck::search_card_from_deck_effect;

    #[test]
    fn search_card_from_deck() {
        let deck_list_tuples = vec![
            (2, 3),   (1, 3),   (114, 2), (112, 2),
            (115, 1), (100, 3), (34, 2),  (56, 3),  (28, 1),
            (45, 3),  (93, 3),  (77, 3),  (112, 1),
            (23, 3),  (111, 3), (91, 1),  (85, 3)
        ];

        let mut player1_shuffled_deck_list = request_shuffle_deck_for_test(&deck_list_tuples);
        println!("{:?}", player1_shuffled_deck_list);
        let number_to_take = 5;
        let mut hand_cards: Vec<i32> = Vec::new();
        let taken_cards = draw_cards(&mut player1_shuffled_deck_list, number_to_take);
        hand_cards.extend_from_slice(&taken_cards);
        println!("{:?}", hand_cards);
        println!("{:?}", player1_shuffled_deck_list);
        let taken_cards = draw_cards(&mut player1_shuffled_deck_list, 1);
        hand_cards.extend_from_slice(&taken_cards);
        println!("뽑은 카드: {:?}", taken_cards);
        println!("남은 덱 : {:?}", player1_shuffled_deck_list);
        println!("현재 핸드 : {:?}", hand_cards);
        player1_shuffled_deck_list=search_card_from_deck_effect(&mut player1_shuffled_deck_list,&mut hand_cards,100,1);
        println!("서치후 남은 덱 : {:?}", player1_shuffled_deck_list);
        println!("서치후 현재 핸드 : {:?}", hand_cards);
        player1_shuffled_deck_list.shuffle(&mut rand::thread_rng());
        println!("서치후 섞은 덱 : {:?}",player1_shuffled_deck_list);


    }
}