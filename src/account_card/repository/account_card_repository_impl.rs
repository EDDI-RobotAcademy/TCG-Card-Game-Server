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
    async fn get_card_list(&self, request: i32) -> Result<Option<Vec<HashMap<i32, i32>>>, Error> {
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
            .filter(|account_card| account_card.account_id == request);

        for card in found_card {
            let mut card_map: HashMap<i32, i32> = HashMap::new();
            card_map.insert(card.card_id, card.card_count);
            card_list.push(card_map);
        }
        Ok(Option::from(card_list))
    }
}
