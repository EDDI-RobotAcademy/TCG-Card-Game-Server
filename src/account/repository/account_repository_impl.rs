use std::sync::Arc;
use async_trait::async_trait;
use diesel::{Connection, MysqlConnection, RunQueryDsl};
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;
use crate::account::entity::account::Account;
use crate::account::repository::account_repository::AccountRepository;
use crate::common::env::env_detector::EnvDetector;
use crate::mysql_config::mysql_connection::MysqlDatabaseConnection;

pub struct AccountRepositoryImpl {
    mysql_database_connection: Arc<AsyncMutex<MysqlDatabaseConnection>>,
}

impl AccountRepositoryImpl {
    pub fn new(mysql_connection: Arc<AsyncMutex<MysqlDatabaseConnection>>) -> Self {
        AccountRepositoryImpl {
            mysql_database_connection: mysql_connection
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<AccountRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<AccountRepositoryImpl>> =
                Arc::new(AsyncMutex::new(AccountRepositoryImpl::new(
                    MysqlDatabaseConnection::get_instance()
            )));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl AccountRepository for AccountRepositoryImpl {

    async fn save(&self, account: Account) {
        use crate::account::entity::account::accounts::dsl::*;

        println!("AccountRepositoryImpl: save()");

        let database_url = EnvDetector::get_mysql_url().expect("DATABASE_URL이 설정되어 있어야 합니다.");
        let mut connection = MysqlConnection::establish(&database_url)
            .expect("Failed to establish a new connection");

        match diesel::insert_into(accounts)
            .values(&account)
            .execute(&mut connection) {
            Ok(_) => println!("Account saved successfully."),
            Err(e) => eprintln!("Error saving account: {:?}", e),
        }
    }
}