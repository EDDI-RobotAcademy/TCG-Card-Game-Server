use std::collections::HashMap;
use async_trait::async_trait;

use crate::card_library::entity::card_dictionary_label::CardDictionaryLabel;

#[async_trait]
pub trait CardLibraryRepository {
    async fn store_dictionary(&mut self, label: CardDictionaryLabel, dictionary: HashMap<String, String>);
    async fn get_dictionary(&self, label: CardDictionaryLabel) -> HashMap<String, String>;
    async fn search_name_by_card_id(&self, card_id: i32) -> String;
}