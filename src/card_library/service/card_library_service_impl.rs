// use std::collections::HashMap;
// use std::env;
// use std::error::Error;
// use std::fs::File;
// use std::sync::Arc;
// use async_trait::async_trait;
// use lazy_static::lazy_static;
// use tokio::sync::Mutex as AsyncMutex;
// use crate::card_library::entity::card_dictionary_label::CardDictionaryLabel;
// use crate::card_library::repository::card_library_repository::CardLibraryRepository;
//
// use crate::card_library::repository::card_library_repository_impl::CardLibraryRepositoryImpl;
// use crate::card_library::service::card_library_service::CardLibraryService;
//
// pub struct CardLibraryServiceImpl {
//     repository: Arc<AsyncMutex<CardLibraryRepositoryImpl>>,
// }
// impl CardLibraryServiceImpl {
//     pub fn new(repository: Arc<AsyncMutex<CardLibraryRepositoryImpl>>) -> Self {
//         CardLibraryServiceImpl { repository}
//     }
//     pub fn get_instance() -> Arc<AsyncMutex<CardLibraryServiceImpl>> {
//         lazy_static! {
//             static ref INSTANCE: Arc<AsyncMutex<CardLibraryServiceImpl>> =
//                 Arc::new(
//                     AsyncMutex::new(
//                         CardLibraryServiceImpl::new(
//                             CardLibraryRepositoryImpl::get_instance())));
//         }
//         INSTANCE.clone()
//     }
//
//     async fn card_data_csv_read(file_path: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
//         let current_exe_path = env::current_exe()?;
//         let current_dir = current_exe_path.parent().ok_or("Unable to get parent directory")?;
//
//         let absolute_path = current_dir.join(file_path);
//         // println!("absolute_path: {:?}", absolute_path);
//
//         let file = File::open(absolute_path)?;
//
//         let mut rdr = csv::Reader::from_reader(file);
//         let mut csv_content: Vec<Vec<String>> = Vec::new();
//
//         for result in rdr.records() {
//             let record = result?;
//             let record_values: Vec<String> = record.iter().map(|field| field.to_string()).collect();
//
//             csv_content.push(record_values);
//         }
//
//         Ok(csv_content)
//     }
//
//     async fn build_card_data_dictionaries(csv_content: &Vec<Vec<String>>) -> (
//         HashMap<String, String>,   // name
//         HashMap<String, String>,   // race
//         HashMap<String, String>,   // grade
//         HashMap<String, String>,   // kind
//         HashMap<String, String>,   // activation_energy
//         HashMap<String, String>,   // attack_point
//         HashMap<String, String>,   // health_point
//         // HashMap<String, String>,   // passive1
//         // HashMap<String, String>,   // passive2
//         // HashMap<String, String>,   // passive3
//         // HashMap<String, String>,   // skill1
//         // HashMap<String, String>,   // skill2
//         // HashMap<String, String>,   // skill2
//         // HashMap<String, String>,   // effect1
//         // HashMap<String, String>,   // effect2
//         // HashMap<String, String>,   // effect3
//     ) {
//         let mut name_dictionary = HashMap::new();
//         let mut race_dictionary = HashMap::new();
//         let mut grade_dictionary = HashMap::new();
//         let mut kind_dictionary = HashMap::new();
//         let mut activation_energy_dictionary = HashMap::new();
//         let mut attack_point_dictionary = HashMap::new();
//         let mut health_point_dictionary = HashMap::new();
//         // let mut passive1_dictionary = HashMap::new();
//         // let mut passive2_dictionary = HashMap::new();
//         // let mut passive3_dictionary = HashMap::new();
//         // let mut skill1_dictionary = HashMap::new();
//         // let mut skill2_dictionary = HashMap::new();
//         // let mut skill3_dictionary = HashMap::new();
//         // let mut effect1_dictionary = HashMap::new();
//         // let mut effect2_dictionary = HashMap::new();
//         // let mut effect3_dictionary = HashMap::new();
//
//         for card_record in csv_content.iter() {
//             let card_number = &card_record[0]; // 카드번호
//             let name = &card_record[1]; // 이름
//             let race = &card_record[2]; // 종족
//             let grade = &card_record[3]; // 등급
//             let kind = &card_record[4]; // 종류
//             // 속성(병종)
//             let activation_energy = &card_record[6]; // 필요_에너지
//             let attack_point = &card_record[7]; // 공격력
//             let health_point = &card_record[8]; // 체력
//             // let passive = &record[12]; // 패시브
//             // let skill = &record[13]; // 스킬
//             // let hp = &record[14]; // 체력
//
//             name_dictionary.insert(card_number.clone(), name.clone());
//             race_dictionary.insert(card_number.clone(), race.clone());
//             grade_dictionary.insert(card_number.clone(), grade.clone());
//             kind_dictionary.insert(card_number.clone(), kind.clone());
//             activation_energy_dictionary.insert(card_number.clone(), activation_energy.clone());
//             attack_point_dictionary.insert(card_number.clone(), attack_point.clone());
//             health_point_dictionary.insert(card_number.clone(), health_point.clone());
//             // passive_dictionary.insert(card_number.clone(), passive.clone());
//             // skill_dictionary.insert(card_number.clone(), skill.clone());
//             // hp_dictionary.insert(card_number.clone(), hp.clone());
//         }
//
//         (
//             name_dictionary,
//             race_dictionary,
//             grade_dictionary,
//             kind_dictionary,
//             activation_energy_dictionary,
//             attack_point_dictionary,
//             health_point_dictionary
//         )
//     }
// }
//
// #[async_trait]
// impl CardLibraryService for CardLibraryServiceImpl {
//     async fn open_library(&self, file_path: &str) {
//         let card_data_csv_content = Self::card_data_csv_read(file_path).await;
//         let prepared_dictionary_hash_tuple =
//             Self::build_card_data_dictionaries(&card_data_csv_content.unwrap()).await;
//
//         let mut card_library_repository_mutex_guard = self.repository.lock().await;
//
//         card_library_repository_mutex_guard.store_dictionary(CardDictionaryLabel::Name, prepared_dictionary_hash_tuple.0).await;
//         card_library_repository_mutex_guard.store_dictionary(CardDictionaryLabel::Race, prepared_dictionary_hash_tuple.1).await;
//         card_library_repository_mutex_guard.store_dictionary(CardDictionaryLabel::Grade, prepared_dictionary_hash_tuple.2).await;
//         card_library_repository_mutex_guard.store_dictionary(CardDictionaryLabel::Kind, prepared_dictionary_hash_tuple.3).await;
//         card_library_repository_mutex_guard.store_dictionary(CardDictionaryLabel::ActivationEnergy, prepared_dictionary_hash_tuple.4).await;
//         card_library_repository_mutex_guard.store_dictionary(CardDictionaryLabel::AttackPoint, prepared_dictionary_hash_tuple.5).await;
//         card_library_repository_mutex_guard.store_dictionary(CardDictionaryLabel::HealthPoint, prepared_dictionary_hash_tuple.6).await;
//
//         drop(card_library_repository_mutex_guard);
//
//         println!("CardLibraryServiceImpl: Card library is now ready.");
//     }
//     async fn get_dictionary(&self, label: CardDictionaryLabel) -> HashMap<String, String> {
//         println!("CardLibraryServiceImpl: get_dictionary()");
//
//         let card_library_repository_mutex_guard = self.repository.lock().await;
//         let found_dictionary = card_library_repository_mutex_guard.get_dictionary(label).await;
//         drop(card_library_repository_mutex_guard);
//
//         found_dictionary
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use tokio::test;
//
//     #[test]
//     async fn test_open_library_and_search() {
//
//         let card_library = CardLibraryServiceImpl::get_instance();
//         let card_library_mutex_guard = card_library.lock().await;
//
//         let file_path = "../../../resources/csv/card_data.csv";
//
//         card_library_mutex_guard.open_library(file_path).await;
//
//         // get specific dictionary
//         let card_library_repository = card_library_mutex_guard.repository.lock().await;
//         let name_dictionary = card_library_repository.get_dictionary(CardDictionaryLabel::Name).await;
//
//         println!("name_dictionary: {:?}", name_dictionary);
//
//         // let kind_dictionary = card_library_mutex_guard.get_dictionary(CardDictionaryLabel::Kind).await;
//         //
//         // println!("kind_dictionary: {:?}", kind_dictionary);
//
//         // card object formation
//         let test_card_id = 17;
//         let test_card_name = card_library_repository.search_name_by_card_id(test_card_id).await;
//         let test_card_race = card_library_repository.search_race_by_card_id(test_card_id).await;
//         let test_card_grade = card_library_repository.search_grade_by_card_id(test_card_id).await;
//         let test_card_kind = card_library_repository.search_kind_by_card_id(test_card_id).await;
//         let test_card_attack_point = card_library_repository.search_attack_point_by_card_id(test_card_id).await;
//         let test_card_health_point = card_library_repository.search_health_point_by_card_id(test_card_id).await;
//
//         println!("Name: {:?}", test_card_name);
//         println!("Race: {:?}", test_card_race);
//         println!("Grade: {:?}", test_card_grade);
//         println!("Kind: {:?}", test_card_kind);
//         println!("Attack Point: {:?}", test_card_attack_point);
//         println!("Health Point: {:?}", test_card_health_point);
//
//         // card list formation with specific label index
//         let target_name = "중갑".to_string();
//         let target_card_id_list = card_library_repository.get_card_list_by_name(target_name).await;
//         let human_race_index = 1;
//         let human_card_list = card_library_repository.get_card_list_by_race(human_race_index).await;
//         let mythical_grade_index = 5;
//         let mythical_card_list = card_library_repository.get_card_list_by_grade(mythical_grade_index).await;
//
//         println!("card_id_list_containing_'중갑': {:?}", target_card_id_list);
//         println!("human_card_list: {:?}", human_card_list);
//         println!("mythical_card_list: {:?}", mythical_card_list);
//     }
// }