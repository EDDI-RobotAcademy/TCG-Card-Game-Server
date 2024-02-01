pub struct VectorToHashConverter;

impl VectorToHashConverter {
    pub fn convert_vector_to_hash(input_list: &Vec<i32>) -> Vec<(i32, i32)> {
        let mut result = Vec::new();
        let mut current_card = input_list[0];
        let mut current_count = 1;

        for &card_id in input_list.iter().skip(1) {
            if card_id == current_card {
                current_count += 1;
            } else {
                result.push((current_card, current_count));
                current_card = card_id;
                current_count = 1;
            }
        }

        result.push((current_card, current_count));

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_vector_to_hash() {
        let input_list = vec![
            19, 8, 8, 8, 9, 9, 25, 25, 25, 27, 27, 27, 151, 20, 20, 20, 2, 2, 2,
            26, 26, 26, 30, 31, 31, 31, 32, 32, 32, 33, 33, 35, 35, 36, 36, 93, 93, 93, 93, 93,
        ];

        let expected_output = vec![
            (19, 1), (8, 3), (9, 2), (25, 3), (27, 3), (151, 1), (20, 3), (2, 3), (26, 3),
            (30, 1), (31, 3), (32, 3), (33, 2), (35, 2), (36, 2), (93, 5),
        ];

        let result = VectorToHashConverter::convert_vector_to_hash(&input_list);
        println!("result:{:?}", result);
        println!("result[0]:{:?}", result[0]);
        println!("result[1]:{:?}", result[1]);

        assert_eq!(result, expected_output);
    }
}