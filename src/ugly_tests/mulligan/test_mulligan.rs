// 사용자는 처음에 받은 손패 5장 중 교체할 카드를 선택 가능
// 사용자가 교체하고자 하는 손패를 선택하여 교체 요청을 하면
// 1. 해당 카드들을 덱에 넣고 셔플
// 2. 들어온 카드 장수만큼 새롭게 손패에 카드를 추가
use crate::ugly_tests::shuffle_deck::test_shuffle_deck::request_shuffle_deck_for_test;
use crate::ugly_tests::draw_deck::test_draw_five_from_deck::draw_cards;
use rand::prelude::SliceRandom;
use rand::thread_rng;

pub(crate) fn mulligan_deck(chosen_card_list: &Vec<i32>, current_deck: Vec<i32>) -> Vec<i32> {
    let mut current_deck_mutable = current_deck.clone();
    current_deck_mutable.extend_from_slice(chosen_card_list);

    println!("Deck after chosen card appended: {:?}", current_deck_mutable);

    current_deck_mutable.shuffle(&mut thread_rng());

    println!("Deck after shuffle again: {:?}", current_deck_mutable);

    current_deck_mutable
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn request_mulligan() {
        let deck_card_list_tuples = vec![
            (7, 3),   (1, 3),   (114, 2), (112, 2),
            (115, 1), (100, 3), (34, 2),  (56, 3),  (28, 1),
            (45, 3),  (93, 3),  (77, 3),  (112, 1),
        ];

        let mut player1_shuffled_deck_list = request_shuffle_deck_for_test(&deck_card_list_tuples);
        let how_many_card_in_player1_deck = player1_shuffled_deck_list.len();

        println!("Player1's current deck:{:?}", player1_shuffled_deck_list);
        println!("How many cards in deck: {} cards", how_many_card_in_player1_deck);

        let mut hand_cards = Vec::new();
        let number_to_take = 5;
        let taken_cards = draw_cards(&mut player1_shuffled_deck_list, number_to_take);
        for id in taken_cards {
            hand_cards.push(id);
        }

        println!("Hand before mulligan: {:?}", hand_cards);
        println!("Deck before mulligan: {:?}", &mut player1_shuffled_deck_list);

        // 실제로는 사용자로부터 전달받은 교체 카드 ID list 일 것임
        // 그러므로 drain 역시 변경이 필요함
        let chosen_cards_to_change = &hand_cards[0..4].to_vec();
        hand_cards.drain(..chosen_cards_to_change.len());

        println!("Chosen to change: {:?}", chosen_cards_to_change);
        println!("Hand after chosen: {:?}", hand_cards);

        let mut renewed_deck = mulligan_deck(chosen_cards_to_change, player1_shuffled_deck_list);

        let retaken_cards = draw_cards(&mut renewed_deck, chosen_cards_to_change.len());
        for id in retaken_cards {
            hand_cards.push(id);
        }

        // println!("Drawn cards by mulligan: {:?}", retaken_cards);
        println!("Deck after mulligan: {:?}", &mut renewed_deck);
        println!("Hand after mulligan: {:?}", hand_cards);
    }
}