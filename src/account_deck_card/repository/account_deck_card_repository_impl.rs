use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use diesel::query_dsl::methods::{FilterDsl, FindDsl};
use diesel::{Connection, MysqlConnection, QueryDsl, ExpressionMethods, RunQueryDsl, OptionalExtension, Insertable};
use diesel::associations::HasTable;
use diesel::result::Error;

use crate::common::env::env_detector::EnvDetector;
use crate::mysql_config::mysql_connection::MysqlDatabaseConnection;

use crate::account_deck_card::entity::account_deck_card::AccountDeckCard;
use crate::account_deck_card::entity::account_deck_card_list::AccountDeckCardList;
use crate::account_deck_card::entity::deck_card::DeckCard;
use crate::account_deck_card::repository::account_deck_card_repository::AccountDeckCardRepository;

pub struct AccountDeckCardRepositoryImpl {
    mysql_database_connection: Arc<AsyncMutex<MysqlDatabaseConnection>>,
}

impl AccountDeckCardRepositoryImpl {
    pub fn new(mysql_connection: Arc<AsyncMutex<MysqlDatabaseConnection>>) -> Self {
        AccountDeckCardRepositoryImpl {
            mysql_database_connection: mysql_connection
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<AccountDeckCardRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<AccountDeckCardRepositoryImpl >> =
                Arc::new(AsyncMutex::new(AccountDeckCardRepositoryImpl::new(
                    MysqlDatabaseConnection::get_instance()
            )));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl AccountDeckCardRepository for AccountDeckCardRepositoryImpl {
    async fn save_deck_card_list(&self, deck_card_list: Vec<AccountDeckCard>) -> Result<String, String> {
        use crate::account_deck_card::entity::account_deck_card::deck_cards::dsl::*;
        println!("AccountDeckCardRepositoryImpl: save()");

        let database_url = EnvDetector::get_mysql_url().expect("DATABASE_URL이 설정되어 있어야 합니다.");
        let mut connection = MysqlConnection::establish(&database_url)
            .expect("Failed to establish a new connection");

        diesel::insert_into(deck_cards::table())
            .values(deck_card_list)
            .execute(&mut connection).expect("덱 저장 실패!");

        Ok("덱 저장에 성공하였습니다.".to_string())
    }

    async fn get_card_list(&self, request_deck_id: i32) -> Result<Option<Vec<HashMap<i32, i32>>>, Error> {
        use crate::account_deck_card::entity::account_deck_card::deck_cards::dsl::*;
        use diesel::query_dsl::filter_dsl::FilterDsl;
        use diesel::prelude::*;

        println!("AccountDeckCardRepositoryImpl: get_card_list()");

        let database_url = EnvDetector::get_mysql_url().expect("DATABASE_URL이 설정되어 있어야 합니다.");
        let mut connection = MysqlConnection::establish(&database_url)
            .expect("Failed to establish a new connection");

        let mut card_list: Vec<HashMap<i32, i32>> = Vec::new();

        let where_clause = FilterDsl::filter(deck_cards, deck_id.eq(deck_id));
        let found_cards = where_clause
            .select((deck_id, card_id, card_count))
            .load::<AccountDeckCard>(&mut connection)?;

        let found_card = found_cards.into_iter()
            .filter(|deck_card| deck_card.deck_id == request_deck_id);

        for card in found_card {
            let mut card_map: HashMap<i32, i32> = HashMap::new();
            card_map.insert(card.card_id, card.card_count);
            card_list.push(card_map);
        }

        Ok(Option::from(card_list))
    }

    async fn get_account_deck_card_list(&self, request_deck_id: i32) -> AccountDeckCardList {
        use crate::account_deck_card::entity::account_deck_card::deck_cards::dsl::*;
        use diesel::query_dsl::filter_dsl::FilterDsl;
        use diesel::prelude::*;

        println!("AccountDeckCardRepositoryImpl: get_account_deck_card_list()");

        let database_url = EnvDetector::get_mysql_url().expect("DATABASE_URL이 설정되어 있어야 합니다.");
        let mut connection = MysqlConnection::establish(&database_url)
            .expect("Failed to establish a new connection");

        let where_clause = FilterDsl::filter(deck_cards, deck_id.eq(deck_id));
        let found_cards = where_clause
            .select((deck_id, card_id, card_count))
            .load::<AccountDeckCard>(&mut connection);

        let found_cards_unwrap = found_cards.unwrap();

        let found_card = found_cards_unwrap.into_iter()
            .filter(|deck_card| deck_card.deck_id == request_deck_id);

        let mut account_card_hashmap = AccountDeckCardList::new();
        for card in found_card {
            let card_list = DeckCard::new(card.card_id, card.card_count);
            account_card_hashmap.add_card(card_list);
        }
        account_card_hashmap
    }

    async fn delete_deck_cards(&self, deck_unique_id: i32) -> Result<(), Error> {
        use crate::account_deck_card::entity::account_deck_card::deck_cards::dsl::*;
        use diesel::query_dsl::filter_dsl::FilterDsl;
        use diesel::prelude::*;

        println!("AccountDeckCardRepositoryImpl: delete_deck_cards()");

        let database_url = EnvDetector::get_mysql_url().expect("DATABASE_URL이 설정되어 있어야 합니다.");
        let mut connection = MysqlConnection::establish(&database_url)
            .expect("Failed to establish a new connection");

        let where_clause = FilterDsl::filter(deck_cards, deck_id.eq(deck_unique_id));

        match diesel::delete(where_clause).execute(&mut connection) {
            Ok(_) => {
                println!("Account Deck cards deleted successfully.");
                Ok(())
            }
            Err(e) => {
                eprintln!("Error delete account deck: {:?}", e);
                Err(e)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::account_deck_card::service::request::account_deck_configuration_request::AccountDeckConfigurationRequest;
    use super::*;

    // DELETE FROM deck_cards WHERE deck_id = 7777;
    #[tokio::test]
    async fn test_save_deck_card_list() {
        let repository = AccountDeckCardRepositoryImpl::get_instance();
        let repository_guard = repository.lock().await;

        let card_id_list = vec![
            19, 8, 8, 8, 9, 9, 25, 25, 25, 27, 27, 27, 151, 20, 20, 20, 2, 2, 2,
            26, 26, 26, 30, 31, 31, 31, 32, 32, 32, 33, 33, 35, 35, 36, 36, 93, 93, 93, 93, 93,
        ];

        let request = AccountDeckConfigurationRequest::new(7777, card_id_list);
        let deck_card_list = request.to_deck_card_list();

        let result = repository_guard.save_deck_card_list(deck_card_list).await.expect("TODO: panic message");
        println!("result: {}", result);
    }
}

