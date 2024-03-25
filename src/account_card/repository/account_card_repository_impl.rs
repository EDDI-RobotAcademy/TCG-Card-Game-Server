use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use diesel::query_dsl::methods::{FilterDsl, FindDsl};
use diesel::{Connection, MysqlConnection, QueryDsl, ExpressionMethods, RunQueryDsl, OptionalExtension, Insertable};
use diesel::associations::HasTable;
use diesel::result::Error;

use crate::common::env::env_detector::EnvDetector;
use crate::account_card::entity::account_card::account_cards::{account_id, columns};
use crate::account_card::entity::account_card::account_cards::dsl::account_cards;
use crate::mysql_config::mysql_connection::MysqlDatabaseConnection;

use crate::account_card::entity::card::Card;
use crate::account_card::entity::account_card::AccountCard;
use crate::account_card::entity::account_card_list::AccountCardList;
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

        println!("AccountCardRepositoryImpl: get_card_list()");

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


    async fn check_same_card(&self, get_card_id : i32, account_card_list: Vec<HashMap<i32, i32>>) -> i32 {

        for account_card in &account_card_list{
            if(account_card.contains_key(&get_card_id)){
                return *account_card.get(&get_card_id).unwrap()
            }
        }

        return 0;

    }

    async fn update_card_count(&self, shop_account_id: i32, shop_update_card: (i32, i32)) -> Result<usize, Error> {
        use crate::account_card::entity::account_card::account_cards::dsl::*;

        use diesel::query_dsl::filter_dsl::FilterDsl;
        use diesel::sql_types::{Integer, Text};
        use diesel::prelude::*;

        println!("AccountCardRepositoryImpl: update_card_count()");

        let database_url = EnvDetector::get_mysql_url().expect("DATABASE_URL이 설정되어 있어야 합니다.");
        let mut connection = MysqlConnection::establish(&database_url)
            .expect("Failed to establish a new connection");

        let select_clause = account_cards.select((account_id, card_id, card_count));
        let where_clause = FilterDsl::filter(account_cards,
            account_id.eq(shop_account_id).and(card_id.eq(shop_update_card.0)));

        let update_count = shop_update_card.1 + 1;

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

    async fn save_new_card(&self, account_unique_id: i32, shop_card_id: i32) -> Result<(), Error> {
        use crate::account_card::entity::account_card::account_cards::dsl::*;

        println!("AccountCardRepositoryImpl: save_new_card()");

        let database_url = EnvDetector::get_mysql_url().expect("DATABASE_URL이 설정되어 있어야 합니다.");
        let mut connection = MysqlConnection::establish(&database_url)
            .expect("Failed to establish a new connection");

        let save_card = AccountCard {
            account_id: account_unique_id,
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
    async fn delete_all_account_cards(&self, account_unique_id: i32) -> Result<(), Error> {
        use crate::account_card::entity::account_card::account_cards::dsl::*;
        use diesel::prelude::*;

        println!("AccountCardRepositoryImpl: delete_all_account_cards()");

        let database_url = EnvDetector::get_mysql_url().expect("DATABASE_URL이 설정되어 있어야 합니다.");
        let mut connection = MysqlConnection::establish(&database_url)
            .expect("Failed to establish a new connection");

        let where_clause = FilterDsl::filter(account_cards, account_id.eq(account_unique_id));

        match diesel::delete(where_clause)
            .execute(&mut connection)
        {
            Ok(_) => {
                println!("All cards of account id : {} removed successfully.", account_unique_id);
                Ok(())
            }
            Err(e) => {
                eprintln!("Error removing account cards: {:?}", e);
                Err(e)
            }
        }
    }

    async fn get_account_card_list(&self, account_unique_id: i32) -> AccountCardList {
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
            .load::<AccountCard>(&mut connection);

        let found_cards_unwrap = found_cards.unwrap();

        let found_card = found_cards_unwrap.into_iter()
            .filter(|account_card| account_card.account_id == account_unique_id);

        let mut account_card_hashmap = AccountCardList::new();
        for card in found_card {
            let card_list = Card::new(card.card_id, card.card_count);
            account_card_hashmap.add_card(card_list);
        }
        account_card_hashmap
    }

    async fn save_account_card(&self, account_card: AccountCard) {
        use crate::account_card::entity::account_card::account_cards::dsl::*;
        println!("AccountDeckCardRepositoryImpl: save()");

        let database_url = EnvDetector::get_mysql_url().expect("DATABASE_URL이 설정되어 있어야 합니다.");
        let mut connection = MysqlConnection::establish(&database_url)
            .expect("Failed to establish a new connection");

        diesel::insert_into(account_cards::table())
            .values(account_card)
            .execute(&mut connection).expect("덱 저장 실패!");
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
    #[test]
    async fn test_delete_account_cards() {
        let repository = AccountCardRepositoryImpl::get_instance();
        let repository_guard = repository.lock().await;

        let card_list = vec![1, 2, 3, 4, 5];
        for card in card_list {
            let save_result = repository_guard.save_new_card(1, card).await;
            assert_eq!((), save_result.unwrap());
        }

        println!("After saved : {:?}", repository_guard.get_card_list(1).await);

        let delete_result = repository_guard.delete_all_account_cards(1).await;

        assert_eq!((), delete_result.unwrap());

        println!("After deleted : {:?}", repository_guard.get_card_list(1).await);
    }
}
