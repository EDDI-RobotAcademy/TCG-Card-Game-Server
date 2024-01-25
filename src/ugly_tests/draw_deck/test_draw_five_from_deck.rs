pub(crate) fn draw_cards(deck: &mut Vec<i32>, number_to_take: usize) -> Vec<i32> {
    deck.drain(..number_to_take).collect()
}

#[cfg(test)]
mod tests {
    use crate::ugly_tests::draw_deck::test_draw_five_from_deck::draw_cards;
    use crate::ugly_tests::shuffle_deck::test_shuffle_deck::request_shuffle_deck_for_test;

    #[test]
    fn draw_five_from_deck() {
        let deck_list_tuples = vec![
            (2, 3),   (1, 3),   (114, 2), (112, 2),
            (115, 1), (100, 3), (34, 2),  (56, 3),  (28, 1),
            (45, 3),  (93, 3),  (77, 3),  (112, 1),
            (23, 3),  (111, 3), (91, 1),  (85, 3)
        ];

        let mut player1_shuffled_deck_list = request_shuffle_deck_for_test(&deck_list_tuples);
        let player2_shuffled_deck_list = request_shuffle_deck_for_test(&deck_list_tuples);

        println!("{:?}", player1_shuffled_deck_list);

        let number_to_take = 5;
        let taken_cards = draw_cards(&mut player1_shuffled_deck_list, number_to_take);

        println!("{:?}", taken_cards);
        println!("{:?}", player1_shuffled_deck_list);
    }
}