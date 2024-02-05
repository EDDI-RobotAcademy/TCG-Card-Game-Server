use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use diesel::query_dsl::methods::{FilterDsl};
use diesel::{Connection, MysqlConnection, QueryDsl, ExpressionMethods, RunQueryDsl, Insertable, BoolExpressionMethods};
use diesel::result::Error;

use crate::account_deck::entity::account_deck::account_decks::{columns};
use crate::account_deck::entity::account_deck::account_decks::dsl::account_decks;

use crate::common::env::env_detector::EnvDetector;
use crate::mysql_config::mysql_connection::MysqlDatabaseConnection;

use crate::account_deck::entity::account_deck::AccountDeck;
use crate::account_deck::repository::account_deck_repository::AccountDeckRepository;
use crate::account_deck::service::request::account_deck_delete_request::AccountDeckDeleteRequest;
use crate::account_deck::service::request::account_deck_modify_request::AccountDeckModifyRequest;

pub struct AccountDeckRepositoryImpl {
    mysql_database_connection: Arc<AsyncMutex<MysqlDatabaseConnection>>,
}

impl AccountDeckRepositoryImpl {
    pub fn new(mysql_connection: Arc<AsyncMutex<MysqlDatabaseConnection>>) -> Self {
        AccountDeckRepositoryImpl {
            mysql_database_connection: mysql_connection
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<AccountDeckRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<AccountDeckRepositoryImpl>> =
                Arc::new(AsyncMutex::new(AccountDeckRepositoryImpl::new(
                    MysqlDatabaseConnection::get_instance()
            )));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl AccountDeckRepository for AccountDeckRepositoryImpl {

    async fn save(&self, deck: AccountDeck) -> Result<(), Error> {
        use crate::account_deck::entity::account_deck::account_decks::dsl::*;

        println!("AccountDeckRepositoryImpl: save()");

        let database_url = EnvDetector::get_mysql_url().expect("DATABASE_URL이 설정되어 있어야 합니다.");
        let mut connection = MysqlConnection::establish(&database_url)
            .expect("Failed to establish a new connection");

        match diesel::insert_into(account_decks)
            .values(&deck)
            .execute(&mut connection)
        {
            Ok(_) => {
                println!("Account Deck saved successfully.");
                Ok(())
            }
            Err(e) => {
                eprintln!("Error saving account deck: {:?}", e);
                Err(e)
            }
        }
    }

    async fn get_list_by_user_int_id(&self, request: i32) -> Result<Option<Vec<HashMap<i32, String>>>, Error> {
        use crate::account_deck::entity::account_deck::account_decks::dsl::*;
        use diesel::query_dsl::filter_dsl::FilterDsl;
        use diesel::prelude::*;

        println!("AccountDeckRepositoryImpl: get_list_by_user_int_id()");

        let database_url = EnvDetector::get_mysql_url().expect("DATABASE_URL이 설정되어 있어야 합니다.");
        let mut connection = MysqlConnection::establish(&database_url)
            .expect("Failed to establish a new connection");

        let mut deck_list: Vec<HashMap<i32, String>> = Vec::new();

        let where_clause = FilterDsl::filter(account_decks, account_id.eq(account_id));
        let found_decks = where_clause
            .select((columns::deck_id, columns::account_id, columns::deck_name))
            .load::<AccountDeck>(&mut connection)?;

        let found_deck = found_decks.into_iter()
            .filter(|account_deck|account_deck.account_id == request);

        for decks in found_deck {
            let mut deck_map: HashMap<i32, String> = HashMap::new();
            deck_map.insert(decks.deck_id, decks.deck_name);
            deck_list.push(deck_map);
        }

        Ok(Option::from(deck_list))
    }

    async fn update(&self, modify_deck: AccountDeckModifyRequest, int_id: i32) -> Result<(), Error> {
        use crate::account_deck::entity::account_deck::account_decks::dsl::*;
        use diesel::prelude::*;

        println!("AccountDeckRepositoryImpl: update_data()");

        let database_url = EnvDetector::get_mysql_url().expect("DATABASE_URL이 설정되어 있어야 합니다.");
        let mut connection = MysqlConnection::establish(&database_url)
            .expect("Failed to establish a new connection");

        let where_clause =
            FilterDsl::filter(account_decks, deck_id.eq(modify_deck.deck_id()).and(account_id.eq(int_id)));

        match diesel::update(where_clause).set(deck_name.eq(modify_deck.deck_name())).execute(&mut connection)
        {
            Ok(_) => {
                println!("Account Deck updated successfully.");
                Ok(())
            }
            Err(e) => {
                eprintln!("Error update account deck: {:?}", e);
                Err(e)
            }
        }
    }

    async fn delete(&self, deck_unique_id: i32) -> Result<(), Error> {
        use crate::account_deck::entity::account_deck::account_decks::dsl::*;
        use diesel::prelude::*;

        println!("AccountDeckRepositoryImpl: delete()");

        let database_url = EnvDetector::get_mysql_url().expect("DATABASE_URL이 설정되어 있어야 합니다.");
        let mut connection = MysqlConnection::establish(&database_url)
            .expect("Failed to establish a new connection");

        let where_clause =
            FilterDsl::filter(account_decks, deck_id.eq(deck_unique_id));
        match diesel::delete(where_clause).execute(&mut connection)
        {
            Ok(_) => {
                println!("Account deck id : {} removed successfully.", deck_unique_id);
                Ok(())
            }
            Err(e) => {
                eprintln!("Error removing account deck: {:?}", e);
                Err(e)
            }
        }
    }
    async fn delete_all_account_decks(&self, account_unique_id: i32) -> Result<(), Error> {
        use crate::account_deck::entity::account_deck::account_decks::dsl::*;
        use diesel::prelude::*;

        println!("AccountDeckRepositoryImpl: delete_all_account_decks()");

        let database_url = EnvDetector::get_mysql_url().expect("DATABASE_URL이 설정되어 있어야 합니다.");
        let mut connection = MysqlConnection::establish(&database_url)
            .expect("Failed to establish a new connection");

        let where_clause =
            FilterDsl::filter(account_decks, account_id.eq(account_unique_id));
        match diesel::delete(where_clause).execute(&mut connection)
        {
            Ok(_) => {
                println!("All decks of account id : {} removed successfully.", account_unique_id);
                Ok(())
            }
            Err(e) => {
                eprintln!("Error removing all account decks: {:?}", e);
                Err(e)
            }
        }
    }
}