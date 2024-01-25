use rand::prelude::SliceRandom;

pub(crate) fn request_shuffle_deck_for_test(deck_list_tuples: &Vec<(i32, i32)>) -> Vec<i32> {
    let mut list_iter: Vec<i32> = Vec::new();

    for (card_no, card_count) in deck_list_tuples {
        for _ in 0..*card_count {
            list_iter.push(*card_no);
        }
    }

    list_iter.shuffle(&mut rand::thread_rng());

    list_iter
}

#[cfg(test)]
mod tests {
    use rand::prelude::SliceRandom;
    use crate::ugly_tests::shuffle_deck::test_shuffle_deck::request_shuffle_deck_for_test;

    #[test]
    fn shuffle_deck() {
        let deck_list_tuples = vec![
            (2, 3),   (1, 3),   (114, 2), (112, 2),
            (115, 1), (100, 3), (34, 2),  (56, 3),  (28, 1),
            (45, 3),  (93, 3),  (77, 3),  (112, 1),
            (23, 3),  (111, 3), (91, 1),  (85, 3)
        ];

        let player1_shuffled_deck_list = request_shuffle_deck_for_test(&deck_list_tuples);
        let player2_shuffled_deck_list = request_shuffle_deck_for_test(&deck_list_tuples);

        println!("{:?}", player1_shuffled_deck_list);
    }
}

