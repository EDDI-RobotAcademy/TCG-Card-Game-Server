use crate::common::converter::vector_to_hash_converter::VectorToHashConverter;
use crate::deck_card::entity::deck_card::DeckCard;

#[derive(Debug, Clone)]
pub struct DeckConfigurationRequest {
    deck_id: i32,
    card_id_list: Vec<i32>
}

impl DeckConfigurationRequest {
    pub fn new(deck_id: i32, card_id_list: Vec<i32>) -> Self {
        DeckConfigurationRequest {
            deck_id,
            card_id_list
        }
    }
    pub fn deck_id(&self) -> i32 { self.deck_id }
    pub fn card_id_list_of_deck(&self) -> &Vec<i32> { &self.card_id_list }

    pub fn to_deck_card_list(&self) -> Vec<DeckCard> {
        let deck_id = self.deck_id();
        let deck_card_id_list = self.card_id_list_of_deck();
        let deck_card_hash = VectorToHashConverter::convert_vector_to_hash(&deck_card_id_list);

        let deck_card_list: Vec<DeckCard> = deck_card_hash
            .iter()
            .map(|(card_id, card_count)| DeckCard::new(deck_id, *card_id, *card_count).unwrap())
            .collect();

        deck_card_list
    }
}

#[cfg(test)]
mod tests {
    use crate::common::converter::vector_to_hash_converter::VectorToHashConverter;
    use super::*;

    #[test]
    fn test_to_deck_card_list() {
        let card_id_list = vec![
            19, 8, 8, 8, 9, 9, 25, 25, 25, 27, 27, 27, 151, 20, 20, 20, 2, 2, 2,
            26, 26, 26, 30, 31, 31, 31, 32, 32, 32, 33, 33, 35, 35, 36, 36, 93, 93, 93, 93, 93,
        ];

        let request = DeckConfigurationRequest::new(1, card_id_list);
        let result = request.to_deck_card_list();

        for card in &result {
            println!("{:?}", card);
        }

        assert_eq!(result.len(), 16);

        assert_eq!(result[0].card_id, 19);
        assert_eq!(result[0].card_count, 1);

        assert_eq!(result[1].card_id, 8);
        assert_eq!(result[1].card_count, 3);
    }
}