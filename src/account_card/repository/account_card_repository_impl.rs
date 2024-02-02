use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use diesel::query_dsl::methods::{FilterDsl, FindDsl};
use diesel::{Connection, MysqlConnection, QueryDsl, ExpressionMethods, RunQueryDsl, OptionalExtension, Insertable};
use diesel::result::Error;

use crate::common::env::env_detector::EnvDetector;
use crate::account_card::entity::account_card::account_cards::columns;
use crate::mysql_config::mysql_connection::MysqlDatabaseConnection;

use crate::account_card::entity::account_card::AccountCard;
use crate::account_card::repository::account_card_repository::AccountCardRepository;


pub struct AccountCardRepositoryImpl {
    mysql_database_connection: Arc<AsyncMutex<MysqlDatabaseConnection>>,
}

impl AccountCardRepositoryImpl {
    pub fn new(mysql_connection: Arc<AsyncMutex<MysqlDatabaseConnection>>) -> Self {
        AccountCardRepositoryImpl {
            mysql_database_connection: mysql_connection
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<AccountCardRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<AccountCardRepositoryImpl>> =
                Arc::new(AsyncMutex::new(AccountCardRepositoryImpl::new(
                    MysqlDatabaseConnection::get_instance()
            )));
        }
        INSTANCE.clone()
    }
}
#[async_trait]
impl AccountCardRepository for AccountCardRepositoryImpl {
    async fn get_card_list(&self, request_account_unique_id: i32) -> Result<Option<Vec<HashMap<i32, i32>>>, Error> {
        use crate::account_card::entity::account_card::account_cards::dsl::*;
        use diesel::query_dsl::filter_dsl::FilterDsl;
        use diesel::prelude::*;

        println!("accountCardRepositoryImpl: get_card_list()");

        let database_url = EnvDetector::get_mysql_url().expect("DATABASE_URL이 설정되어 있어야 합니다.");
        let mut connection = MysqlConnection::establish(&database_url)
            .expect("Failed to establish a new connection");

        let mut card_list: Vec<HashMap<i32, i32>> = Vec::new();

        let where_clause = FilterDsl::filter(account_cards, account_id.eq(account_id));
        let found_cards = where_clause
            .select((account_id, card_id, card_count))
            .load::<AccountCard>(&mut connection)?;

        let found_card = found_cards.into_iter()
            .filter(|account_card| account_card.account_id == request_account_unique_id);

        for card in found_card {
            let mut card_map: HashMap<i32, i32> = HashMap::new();
            card_map.insert(card.card_id, card.card_count);
            card_list.push(card_map);
        }
        Ok(Option::from(card_list))
    }


    async fn check_same_card(&self, get_card_list: Vec<i32>, account_card_list: Vec<HashMap<i32, i32>>) -> HashMap<i32, bool> {

        let mut account_card_check : HashMap<i32, bool> = Default::default();

        for get_card in get_card_list {
            account_card_check.insert(get_card, false);
            for account_card in &account_card_list{
                if(account_card.contains_key(&get_card)){
                    account_card_check.insert(get_card, true);
                }
            }
        }
        account_card_check

    }

    async fn update_card_count(&self, shop_account_id: i32, shop_card_id: i32) -> Result<usize, diesel::result::Error> {
        use crate::account_card::entity::account_card::account_cards::dsl::*;

        use diesel::query_dsl::filter_dsl::FilterDsl;
        use diesel::sql_types::{Integer, Text};
        use diesel::prelude::*;

        println!("AccountCardRepositoryImpl: update_card_count()");

        let database_url = EnvDetector::get_mysql_url().expect("DATABASE_URL이 설정되어 있어야 합니다.");
        let mut connection = MysqlConnection::establish(&database_url)
            .expect("Failed to establish a new connection");

        let select_clause = account_cards.select((account_id, card_id, card_count));
        let where_clause = FilterDsl::filter(account_cards, account_id.eq(shop_account_id).and(card_id.eq(shop_card_id)));

        let found_account_cards = where_clause
            .select((account_id, card_id, card_count))
            .load::<AccountCard>(&mut connection)
            .expect("error");


        let update_count = found_account_cards[0].card_count + 1;

        match diesel::update(where_clause)
            .set((columns::card_count.eq(update_count)))
            .execute(&mut connection)
        {
            Ok(num) => {
                println!("card count updated successfully.");
                Ok(num)
            }
            Err(e) => {
                eprintln!("Error updating card count: {:?}", e);
                Err(e)
            }
        }

    }

    async fn save_new_card(&self, shop_account_id: i32, shop_card_id: i32) -> Result<(), diesel::result::Error> {
        use crate::account_card::entity::account_card::account_cards::dsl::*;

        println!("AccountCardRepositoryImpl: save_new_card()");

        let database_url = EnvDetector::get_mysql_url().expect("DATABASE_URL이 설정되어 있어야 합니다.");
        let mut connection = MysqlConnection::establish(&database_url)
            .expect("Failed to establish a new connection");

        let save_card = AccountCard {
            account_id: shop_account_id,
            card_id: shop_card_id,
            card_count: 1,
        };

        match diesel::insert_into(account_cards)
            .values(&save_card)
            .execute(&mut connection)
        {
            Ok(_) => {
                println!("new card saved successfully.");
                Ok(())
            }
            Err(e) => {
                eprintln!("Error saving new card: {:?}", e);
                Err(e)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;
    use tokio::time::timeout;

    #[tokio::test]
    async fn test_get_account_card_list() {
        // 사용자가 가지고 있는 카드 리스트
        let repository = AccountCardRepositoryImpl::get_instance();
        let repository_guard = repository.lock().await;

        let result = repository_guard.get_card_list(5).await;

        match repository_guard.get_card_list(5).await {
            Ok(result) => {
                assert!(result.is_some(), "Expected Some, but got None");

                if let Some(card_list) = result {
                    println!("Card List: {:?}", card_list);
                    assert!(!card_list.is_empty(), "Expected non-empty card list");
                }
            }
            Err(err) => {
                println!("Error: {:?}", err);
                panic!("Test failed with error: {:?}", err);
            }
        }
    }
}
