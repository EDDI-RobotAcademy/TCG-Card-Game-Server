use std::sync::Arc;
use async_trait::async_trait;
use bcrypt::hash;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;

use diesel::query_dsl::methods::{FilterDsl, FindDsl};
use diesel::{Connection, MysqlConnection, QueryDsl, ExpressionMethods, RunQueryDsl, OptionalExtension};

use diesel::dsl::Eq;
use diesel::sql_types::Text;

use crate::account::entity::account::Account;
use crate::account::entity::account::accounts::columns;
use crate::account::entity::account::accounts::dsl::accounts;
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

    async fn save(&self, account: Account) -> Result<(), diesel::result::Error> {
        use crate::account::entity::account::accounts::dsl::*;

        println!("AccountRepositoryImpl: save()");

        let database_url = EnvDetector::get_mysql_url().expect("DATABASE_URL이 설정되어 있어야 합니다.");
        let mut connection = MysqlConnection::establish(&database_url)
            .expect("Failed to establish a new connection");

        match diesel::insert_into(accounts)
            .values(&account)
            .execute(&mut connection)
        {
            Ok(_) => {
                println!("Account saved successfully.");
                Ok(())
            }
            Err(e) => {
                eprintln!("Error saving account: {:?}", e);
                Err(e)
            }
        }
    }

    async fn find_by_user_id(&self, account_user_id: &str) -> Result<Option<Account>, diesel::result::Error> {
        use crate::account::entity::account::accounts::dsl::*;
        use diesel::query_dsl::filter_dsl::FilterDsl;
        use diesel::sql_types::{Integer, Text};
        use diesel::prelude::*;

        println!("AccountRepositoryImpl: find_by_user_id()");

        let database_url = EnvDetector::get_mysql_url().expect("DATABASE_URL이 설정되어 있어야 합니다.");
        let mut connection = MysqlConnection::establish(&database_url)
            .expect("Failed to establish a new connection");

        let select_clause = accounts.select((columns::id, columns::user_id, columns::password, columns::gold));
        let where_clause = FilterDsl::filter(accounts, columns::user_id.eq(user_id));
        let found_accounts = where_clause
            .select((columns::id, columns::user_id, columns::password, columns::gold))
            .load::<Account>(&mut connection)?;

        let found_account = found_accounts
            .into_iter()
            .find(|account| account.user_id == account_user_id);

        Ok(Option::from(found_account))
    }

    async fn delete(&self, account: Account) -> Result<(), diesel::result::Error> {
        use crate::account::entity::account::accounts::dsl::*;

        println!("AccountRepositoryImpl: delete()");

        let database_url = EnvDetector::get_mysql_url().expect("DATABASE_URL이 설정되어 있어야 합니다.");
        let mut connection = MysqlConnection::establish(&database_url)
            .expect("Failed to establish a new connection");

        match diesel::delete(FilterDsl::filter(accounts, columns::user_id.eq(account.user_id())))

            .execute(&mut connection)
        {
            Ok(_) => {
                println!("Account deleted successfully.");
                Ok(())
            }
            Err(e) => {
                eprintln!("Error deleting account: {:?}", e);
                Err(e)
            }
        }
    }

    async fn update(&self, account: Account, account_new_password: &str) -> Result<usize, diesel::result::Error> {

        println!("AccountRepositoryImpl: update()");

        let database_url = EnvDetector::get_mysql_url().expect("DATABASE_URL이 설정되어 있어야 합니다.");
        let mut connection = MysqlConnection::establish(&database_url)
            .expect("Failed to establish a new connection");
        // 넘겨 받은 새로운 비밀번호를 암호화 합니다.
        let hashed_password = hash(account_new_password, 12).expect("hashed_password fail");

        match diesel::update(FilterDsl::filter(accounts, columns::user_id.eq(account.user_id)))
            .set((
                columns::password.eq(hashed_password),
            ))
            .execute(&mut connection)
        {
            Ok(num) => {
                println!("Account updated successfully.");
                Ok(num)
            }
            Err(e) => {
                eprintln!("Error updating account: {:?}", e);
                Err(e)
            }
        }
    }
}
