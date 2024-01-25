// [(2, 3), (1, 3), (114, 2), (115, 1)] <- 40

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, HashMap};
    use indexmap::IndexMap;

    #[test]
    fn tuple_save_with_hashmap() {
        let deck_list_tuples = vec![
                                            (2, 3),   (1, 3),   (114, 2), (112, 2),
                                            (115, 1), (100, 3), (34, 2),  (56, 3),  (28, 1),
                                            (45, 3),  (93, 3),  (77, 3),  (112, 1),
                                            (23, 3),  (111, 3), (91, 1),  (85, 3)
                                        ];

        let deck_list_hashmap: HashMap<_, _> = deck_list_tuples.into_iter().collect();

        for (key, value) in &deck_list_hashmap {
            println!("Key: {}, Value: {}", key, value);
        }
    }

    #[test]
    fn tuple_save_with_btree() {
        let deck_list_tuples = vec![
            (2, 3),   (1, 3),   (114, 2), (112, 2),
            (115, 1), (100, 3), (34, 2),  (56, 3),  (28, 1),
            (45, 3),  (93, 3),  (77, 3),  (112, 1),
            (23, 3),  (111, 3), (91, 1),  (85, 3)
        ];

        let deck_list_btreemap: BTreeMap<_, _> = deck_list_tuples.into_iter().collect();

        for (key, value) in &deck_list_btreemap {
            println!("Key: {}, Value: {}", key, value);
        }
    }

    #[test]
    fn tuple_save_with_indexMap() {
        let deck_list_tuples = vec![
            (2, 3), (1, 3), (114, 2), (112, 2),
            (115, 1), (100, 3), (34, 2), (56, 3), (28, 1),
            (45, 3), (93, 3), (77, 3), (112, 1),
            (23, 3), (111, 3), (91, 1), (85, 3),
        ];

        let mut deck_list_indexmap = IndexMap::new();

        for (key, value) in deck_list_tuples {
            deck_list_indexmap.insert(key, value);
        }

        for (key, value) in &deck_list_indexmap {
            println!("Key: {}, Value: {}", key, value);
        }
    }
}

