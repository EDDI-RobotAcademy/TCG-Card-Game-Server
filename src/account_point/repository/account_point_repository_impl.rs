use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;

use tokio::sync::Mutex as AsyncMutex;

use diesel::query_dsl::methods::{FilterDsl, FindDsl};
use diesel::{Connection, MysqlConnection, QueryDsl, ExpressionMethods, RunQueryDsl};

use crate::account_point::entity::account_point::AccountPoint;
use crate::account_point::entity::account_point::account_points::dsl::{account_id, account_points};
use crate::account_point::entity::account_point::account_points::columns;
use crate::account_point::repository::account_point_repository::AccountPointRepository;
use crate::common::env::env_detector::EnvDetector;
use crate::mysql_config::mysql_connection::MysqlDatabaseConnection;

pub struct AccountPointRepositoryImpl {
    mysql_database_connection: Arc<AsyncMutex<MysqlDatabaseConnection>>,
}

impl AccountPointRepositoryImpl {
    pub fn new(mysql_connection: Arc<AsyncMutex<MysqlDatabaseConnection>>) -> Self {
        AccountPointRepositoryImpl {
            mysql_database_connection: mysql_connection
        }
    }

    pub fn get_instance() -> Arc<AsyncMutex<AccountPointRepositoryImpl>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<AccountPointRepositoryImpl>> =
                Arc::new(AsyncMutex::new(AccountPointRepositoryImpl::new(
                    MysqlDatabaseConnection::get_instance()
            )));
        }
        INSTANCE.clone()
    }
}

#[async_trait]
impl AccountPointRepository for AccountPointRepositoryImpl {
    async fn update_gold(&self, account_point:AccountPoint, golds: i32) -> Result<usize, diesel::result::Error> {
        println!("AccountPointRepositoryImpl: update_gold()");

        let database_url = EnvDetector::get_mysql_url().expect("DATABASE_URL이 설정되어 있어야 합니다.");
        let mut connection = MysqlConnection::establish(&database_url)
            .expect("Failed to establish a new connection");

        match diesel::update(FilterDsl::filter(account_points, columns::account_id.eq(account_point.account_id)))
            .set((
                columns::gold.eq(golds),
            ))
            .execute(&mut connection)
        {
            Ok(num) => {
                println!("Gold points updated successfully.");
                Ok(num)
            }
            Err(e) => {
                eprintln!("Error updating Gold points: {:?}", e);
                Err(e)
            }
        }
    }

//     async fn pay_gold(&self, account: Account, account_user_id: &str, pay_golds: i32) -> Result<usize, diesel::result::Error> {
//         println!("AccountPointRepositoryImpl: pay_gold()");
//
//         let database_url = EnvDetector::get_mysql_url().expect("DATABASE_URL이 설정되어 있어야 합니다.");
//         let mut connection = MysqlConnection::establish(&database_url)
//             .expect("Failed to establish a new connection");
//
//         let where_clause = FilterDsl::filter(accounts, columns::user_id.eq(user_id));
//         let found_accounts = where_clause
//             .select((columns::id, columns::user_id, columns::password, columns::gold))
//             .load::<Account>(&mut connection)?;
//
//         let found_account = found_accounts
//             .into_iter()
//             .find(|account| account.user_id == account_user_id);
//
//         let current_gold = found_account.unwrap().gold;
//
//         if current_gold >= pay_golds {
//             let result_gold = current_gold - pay_golds;
//
//             match diesel::update(FilterDsl::filter(accounts, columns::user_id.eq(account.user_id)))
//                 .set((
//                     columns::gold.eq(result_gold),
//                 ))
//                 .execute(&mut connection)
//             {
//                 Ok(num) => {
//                     println!("Gold of account updated successfully.");
//                     Ok(num)
//                 }
//                 Err(e) => {
//                     eprintln!("Error updating gold of account: {:?}", e);
//                     Err(e)
//                 }
//             }
//         } else {
//             println!("You don't have not enough gold");
//             Err(diesel::result::Error::NotFound)
//         }
//     }
}