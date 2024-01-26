

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::io;
    use rand::Rng;
    use crate::common::csv::csv_reader::{build_dictionaries, csv_read, get_card_kinds};
    use crate::ugly_tests::draw_deck::test_draw_five_from_deck::draw_cards;
    use crate::ugly_tests::shuffle_deck::test_shuffle_deck::request_shuffle_deck_for_test;

    #[test]
    fn check_card_type() {

    }

    #[test]
    fn draw_five_from_deck() {
        let filename = "../../../resources/csv/every_card.csv";
        let (
            mut race_dictionary,
            mut card_grade_dictionary,
            mut card_kinds_dictionary,
            mut energy_needed_dictionary,
            mut attack_dictionary,
            mut passive_dictionary,
            mut skill_dictionary,
            mut hp_dictionary,
        ) = (HashMap::new(), HashMap::new(), HashMap::new(), HashMap::new(), HashMap::new(), HashMap::new(), HashMap::new(), HashMap::new());

        match csv_read(filename) {
            Ok((csv_content)) => {
                println!("CSV file successfully processed.");

                (
                    race_dictionary,
                    card_grade_dictionary,
                    card_kinds_dictionary,
                    energy_needed_dictionary,
                    attack_dictionary,
                    passive_dictionary,
                    skill_dictionary,
                    hp_dictionary,
                ) = build_dictionaries(&csv_content);
            }
            Err(err) => eprintln!("Error: {}", err),
        }

        let card_number_to_test = "5";

        match get_card_kinds(card_number_to_test, &card_kinds_dictionary) {
            Some(card_kinds) => println!("Card Kinds for {} : {}", card_number_to_test, card_kinds),
            None => println!("Card not found: {}", card_number_to_test),
        }

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
        let mut hand_cards: Vec<i32> = Vec::new();

        // Draw initial 5 cards
        let taken_cards = draw_cards(&mut player1_shuffled_deck_list, number_to_take);
        hand_cards.extend_from_slice(&taken_cards);

        println!("{:?}", hand_cards);
        println!("{:?}", player1_shuffled_deck_list);

        let mut turn = 1;

        let battle_field_unit_list: Vec<i32>;

        while !player1_shuffled_deck_list.is_empty() {
            // let mut user_selection_card = String::new();

            println!("Turn: {}", turn);
            // Draw 1 card per turn
            let taken_cards = draw_cards(&mut player1_shuffled_deck_list, 1);

            // Add the drawn card to hand
            hand_cards.extend_from_slice(&taken_cards);

            println!("Drawn Cards: {:?}", taken_cards);
            println!("Remaining Cards: {:?}", player1_shuffled_deck_list);
            println!("Hand Cards: {:?}", hand_cards);

            // print!("사용할 카드 번호를 입력하세요: ");
            // io::stdin().read_line(&mut user_selection_card).expect("Failed to read line");

            // let user_selection_card_number: i32 = user_selection_card.trim().parse().expect("숫자값을 넣어야 합니다.");
            // println!("{} 번 카드를 사용합니다.", user_selection_card_number);

            if !hand_cards.is_empty() {
                let random_card_index = rand::thread_rng().gen_range(0..hand_cards.len());
                let selected_card = hand_cards.remove(random_card_index);

                println!("{} 번 카드를 사용합니다", selected_card);
            }

            turn += 1;
        }

        println!("덱사 발생!")
    }
}