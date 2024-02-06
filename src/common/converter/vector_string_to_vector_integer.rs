pub struct VectorStringToVectorInteger;

impl VectorStringToVectorInteger {
    pub fn vector_string_to_vector_i32(card_string_list: Vec<String>) -> Vec<i32> {
        let mut output : Vec<i32> = Vec::new();
        for string_type_card_id in card_string_list {
            if let Ok(integer_type_card_id) = string_type_card_id.parse::<i32>() {
                output.push(integer_type_card_id);
            } else {
                eprintln!("Failed to parse the card id");
            }
        }
        output
    }
}