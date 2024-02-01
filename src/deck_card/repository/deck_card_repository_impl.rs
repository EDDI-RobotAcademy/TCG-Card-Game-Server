use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use diesel::query_dsl::methods::{FilterDsl, FindDsl};
use diesel::{Connection, MysqlConnection, QueryDsl, ExpressionMethods, RunQueryDsl, OptionalExtension, Insertable};
use diesel::result::Error;

use crate::common::env::env_detector::EnvDetector;
use crate::deck_card::entity::deck_card::deck_cards::columns;
use crate::mysql_config::mysql_connection::MysqlDatabaseConnection;

use crate::deck_card::entity::deck_card::DeckCard;
use crate::deck_card::repository::deck_card_repository::DeckCardRepository;
use crate::deck_card::service::request::deck_configuration_request::DeckConfigurationRequest;

pub struct DeckCardRepositoryImpl {
    mysql_database_connection: Arc<AsyncMutex<MysqlDatabaseConnection>>,
}

impl DeckCardRepositoryImpl {
    pub fn new(mysql_connection: Arc<AsyncMutex<MysqlDatabaseConnection>>) -> Self {
        DeckCardRepositoryImpl {
            mysql_database_connection: mysql_connection
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<DeckCardRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<DeckCardRepositoryImpl>> =
                Arc::new(AsyncMutex::new(DeckCardRepositoryImpl::new(
                    MysqlDatabaseConnection::get_instance()
            )));
        }
        INSTANCE.clone()
    }
    async fn check_this_deck_comply_rules(&self,deck: &Vec<i32>,map: HashMap<&i32, i32>) -> Result<(), String> {
        match deck.len() {
            40 => {
                println!("대박!! 40장을 정확히 맞추셨습니다!!");
            }
            length => {
                // 40장이 아닌 경우
                let error_string = format!("덱에 총 {}장이 있습니다. 정확히 40장을 맞춰주세요!", length);
                println!("{}", error_string);
                return Err(error_string)
            }
        }
         let mut card_count_map = map;
        for (key, value) in card_count_map.iter() {

            if *value > 3 {
                let error_string = format!("{}번 카드가 3장 이상입니다. 같은 카드는 덱에 3장 이하여야 합니다!", key);
                println!("{}", error_string);
                return Err(error_string);
            }
        }
        Ok(())
    }
}

#[async_trait]
impl DeckCardRepository for DeckCardRepositoryImpl {
    async fn save(&self, deck: DeckConfigurationRequest) -> Result<String, String> {
        use crate::deck_card::entity::deck_card::deck_cards::dsl::*;
        println!("DeckCardRepositoryImpl: save()");

        // TODO: 카드의 등급 제한을 넘기지 않도록 덱을 구성했는지 검사하는 로직 추가해야 함

        let database_url = EnvDetector::get_mysql_url().expect("DATABASE_URL이 설정되어 있어야 합니다.");
        let mut connection = MysqlConnection::establish(&database_url)
            .expect("Failed to establish a new connection");
        let mut card_count_map = HashMap::new();
        let mut built_deck=deck.card_id_list_of_deck().clone();


        let mut bare_card_id_list_for_loop = deck.card_id_list_of_deck().clone();

        for card_key in bare_card_id_list_for_loop.iter() {
            let card_counts = card_count_map.entry(card_key).or_insert(0);
            *card_counts += 1;
        }
        let  result =self.check_this_deck_comply_rules(&built_deck,card_count_map.clone()).await;
        if result.is_err()
        {
            return Err(result.unwrap_err())
        }

        else {
            let mut bare_card_id_list_to_remove_duplication = deck.card_id_list_of_deck().clone();
            let unique_card_id_set: HashSet<i32> = bare_card_id_list_to_remove_duplication.into_iter().collect();
            let unique_card_id_list: Vec<i32> = unique_card_id_set.into_iter().collect();

            println!("{:?}", unique_card_id_list);

            for card_index in unique_card_id_list {
                let card_count_index = card_count_map.get(&card_index).expect("REASON").clone();
                let deck_card = DeckCard::new(deck.deck_id(), card_index, card_count_index).unwrap();
                diesel::insert_into(deck_cards)
                    .values(&deck_card)
                    .execute(&mut connection)
                    .expect("REASON");
            }

            Ok("덱 저장에 성공하였습니다.".to_string())
        }
    }

    async fn get_card_list(&self, request: i32) -> Result<Option<Vec<HashMap<i32, i32>>>, Error> {
        use crate::deck_card::entity::deck_card::deck_cards::dsl::*;
        use diesel::query_dsl::filter_dsl::FilterDsl;
        use diesel::prelude::*;

        println!("DeckCardRepositoryImpl: get_card_list()");

        let database_url = EnvDetector::get_mysql_url().expect("DATABASE_URL이 설정되어 있어야 합니다.");
        let mut connection = MysqlConnection::establish(&database_url)
            .expect("Failed to establish a new connection");

        let mut card_list: Vec<HashMap<i32, i32>> = Vec::new();

        let where_clause = FilterDsl::filter(deck_cards, deck_id.eq(deck_id));
        let found_cards = where_clause
            .select((deck_id, card_id, card_count))
            .load::<DeckCard>(&mut connection)?;

        let found_card = found_cards.into_iter()
            .filter(|deck_card| deck_card.deck_id == request);

        for card in found_card {
            let mut card_map: HashMap<i32, i32> = HashMap::new();
            card_map.insert(card.card_id, card.card_count);
            card_list.push(card_map);
        }

        Ok(Option::from(card_list))
    }
}

