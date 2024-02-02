use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use tokio::sync::Mutex as AsyncMutex;
use crate::card_library::entity::card_dictionary_label::CardDictionaryLabel;
use crate::card_library::repository::card_library_repository::CardLibraryRepository;

pub struct CardLibraryRepositoryImpl {
    labeled_card_dictionary_hash: Arc<AsyncMutex<HashMap<CardDictionaryLabel, HashMap<String, String>>>>,
}
impl CardLibraryRepositoryImpl {
    pub fn new() -> Self {
        CardLibraryRepositoryImpl { labeled_card_dictionary_hash: Arc::new(AsyncMutex::new(HashMap::new())) }
    }
    pub fn get_instance() -> Arc<AsyncMutex<CardLibraryRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<CardLibraryRepositoryImpl>> =
                Arc::new(
                    AsyncMutex::new(
                        CardLibraryRepositoryImpl::new()));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl CardLibraryRepository for CardLibraryRepositoryImpl {
    async fn store_dictionary(&mut self, label: CardDictionaryLabel, dictionary: HashMap<String, String>) {
        let mut labeled_card_dictionary_hash_guard = self.labeled_card_dictionary_hash.lock().await;
        labeled_card_dictionary_hash_guard.insert(label, dictionary);
    }
    async fn get_dictionary(&self, label: CardDictionaryLabel) -> HashMap<String, String> {
        let labeled_dictionaries = self.labeled_card_dictionary_hash.lock().await;
        let selected_dictionary = labeled_dictionaries.get(&label).unwrap().clone();
        selected_dictionary
    }
    async fn search_name_by_card_id(&self, card_id: i32) -> String {
        let name_dictionary = self.get_dictionary(CardDictionaryLabel::Name).await;
        let card_id_into_key = format!("{}", card_id);
        let result = name_dictionary.get(&card_id_into_key).unwrap();
        result.to_string()
    }
    async fn search_race_by_card_id(&self, card_id: i32) -> String {
        let race_dictionary = self.get_dictionary(CardDictionaryLabel::Race).await;
        let card_id_into_key = format!("{}", card_id);
        let result = race_dictionary.get(&card_id_into_key).unwrap();
        result.to_string()
    }
    async fn search_grade_by_card_id(&self, card_id: i32) -> String {
        let grade_dictionary = self.get_dictionary(CardDictionaryLabel::Grade).await;
        let card_id_into_key = format!("{}", card_id);
        let result = grade_dictionary.get(&card_id_into_key).unwrap();
        result.to_string()
    }
    async fn search_kind_by_card_id(&self, card_id: i32) -> String {
        let kind_dictionary = self.get_dictionary(CardDictionaryLabel::Kind).await;
        let card_id_into_key = format!("{}", card_id);
        let result = kind_dictionary.get(&card_id_into_key).unwrap();
        result.to_string()
    }
    async fn search_attack_point_by_card_id(&self, card_id: i32) -> String {
        let attack_point_dictionary = self.get_dictionary(CardDictionaryLabel::AttackPoint).await;
        let card_id_into_key = format!("{}", card_id);
        let result = attack_point_dictionary.get(&card_id_into_key).unwrap();
        result.to_string()
    }
    async fn search_health_point_by_card_id(&self, card_id: i32) -> String {
        let health_point_dictionary = self.get_dictionary(CardDictionaryLabel::HealthPoint).await;
        let card_id_into_key = format!("{}", card_id);
        let result = health_point_dictionary.get(&card_id_into_key).unwrap();
        result.to_string()
    }
    async fn get_card_list_by_name(&self, card_name: String) -> Vec<String> {
        let name_dictionary = self.get_dictionary(CardDictionaryLabel::Name).await;
        let mut card_list_containing_name = Vec::new();
        for (key, value) in name_dictionary {
            if value.contains(&card_name) {
                card_list_containing_name.push(key);
            }
        }
        card_list_containing_name
    }
    async fn get_card_list_by_race(&self, race_index: i32) -> Vec<String> {
        let race_dictionary = self.get_dictionary(CardDictionaryLabel::Race).await;
        let mut card_list_matched_with_index = Vec::new();
        for (key, value) in race_dictionary {
            if value == format!("{}", race_index) {
                card_list_matched_with_index.push(key)
            }
        }
        card_list_matched_with_index
    }
    async fn get_card_list_by_grade(&self, grade_index: i32) -> Vec<String> {
        let grade_dictionary = self.get_dictionary(CardDictionaryLabel::Grade).await;
        let mut card_list_matched_with_index = Vec::new();
        for (key, value) in grade_dictionary {
            if value == format!("{}", grade_index) {
                card_list_matched_with_index.push(key)
            }
        }
        card_list_matched_with_index
    }
    async fn get_card_list_by_kind(&self, kind_index: i32) -> Vec<String> {
        let kind_dictionary = self.get_dictionary(CardDictionaryLabel::Kind).await;
        let mut card_list_matched_with_index = Vec::new();
        for (key, value) in kind_dictionary {
            if value == format!("{}", kind_index) {
                card_list_matched_with_index.push(key)
            }
        }
        card_list_matched_with_index
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[test]
    async fn test_store_and_get_dictionary() {
        let mut sample_dictionary = HashMap::new();
        sample_dictionary.insert("1".to_string(), "넘쳐흐르는 사기".to_string());

        let label = CardDictionaryLabel::Name;

        let card_library_repository_mutex = CardLibraryRepositoryImpl::get_instance();
        let mut card_library_repository_mutex_guard = card_library_repository_mutex.lock().await;

        card_library_repository_mutex_guard.store_dictionary(label, sample_dictionary).await;
        let name_dictionary = card_library_repository_mutex_guard.get_dictionary(label).await;

        println!("{:?}", name_dictionary);
    }
}