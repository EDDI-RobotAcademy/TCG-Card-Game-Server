use std::collections::HashMap;

pub struct HashToVectorConverter;

impl HashToVectorConverter {
    fn hash_vector_to_vector(hash_list: Vec<HashMap<i32, i32>>) -> Vec<i32> {
        hash_list
            .into_iter()
            .flat_map(|entry| {
                entry.into_iter().flat_map(|(key, value)| vec![key; value as usize])
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_vector_to_vector() {
        let account_deck_list = vec![
            HashMap::from([(19, 1)]),
            HashMap::from([(8, 3)]),
            HashMap::from([(9, 2)]),
            HashMap::from([(25, 3)]),
            HashMap::from([(27, 3)]),
            HashMap::from([(151, 1)]),
            HashMap::from([(20, 3)]),
            HashMap::from([(2, 3)]),
            HashMap::from([(26, 3)]),
            HashMap::from([(30, 1)]),
            HashMap::from([(31, 3)]),
            HashMap::from([(32, 3)]),
            HashMap::from([(33, 2)]),
            HashMap::from([(35, 2)]),
            HashMap::from([(36, 2)]),
            HashMap::from([(93, 5)]),
        ];

        let transformed_list = HashToVectorConverter::hash_vector_to_vector(account_deck_list);

        assert_eq!(
            transformed_list,
            vec![
                19, 8, 8, 8, 9, 9, 25, 25, 25, 27, 27, 27, 151, 20, 20, 20, 2, 2, 2, 26, 26, 26,
                30, 31, 31, 31, 32, 32, 32, 33, 33, 35, 35, 36, 36, 93, 93, 93, 93, 93
            ]
        );
    }
}