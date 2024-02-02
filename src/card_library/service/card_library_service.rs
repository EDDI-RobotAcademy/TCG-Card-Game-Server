use std::collections::HashMap;
use async_trait::async_trait;
use crate::card_library::entity::card_dictionary_label::CardDictionaryLabel;

#[async_trait]
pub trait CardLibraryService {
    async fn open_library(&self, file_path: &str);
    async fn get_dictionary(&self, label: CardDictionaryLabel) -> HashMap<String, String>;
}