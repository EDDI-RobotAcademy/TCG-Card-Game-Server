use std::ops::Deref;
use std::sync::Arc;
use diesel::{Connection, MysqlConnection};
use dotenv::dotenv;
use lazy_static::lazy_static;
use crate::common::env::env_detector::EnvDetector;
use tokio::sync::Mutex as AsyncMutex;

pub struct MysqlDatabaseConnection {
    // mysql_connection: Arc<AsyncMutex<MysqlConnection>>,
    mysql_connection: MysqlConnection,
}

impl MysqlDatabaseConnection {
    fn new() -> Self {
        dotenv().ok();

        let database_url = EnvDetector::get_mysql_url().expect("DATABASE_URL이 설정되어 있어야 합니다.");
        let mysql_connection = MysqlConnection::establish(&database_url)
            .expect(&format!("연결에 실패했습니다. {}", database_url));

        // let mysql_connection_mutex = Arc::new(AsyncMutex::new(mysql_connection));
        //
        // MysqlDatabaseConnection { mysql_connection: mysql_connection_mutex }
        MysqlDatabaseConnection { mysql_connection }
    }

    pub fn get_instance() -> Arc<AsyncMutex<MysqlDatabaseConnection>> {
        lazy_static! {
            static ref INSTANCE: Arc<AsyncMutex<MysqlDatabaseConnection>> =
                Arc::new(AsyncMutex::new(MysqlDatabaseConnection::new()));
        }
        INSTANCE.clone()
    }
}

impl Deref for MysqlDatabaseConnection {
    type Target = MysqlConnection;

    fn deref(&self) -> &Self::Target {
        &self.mysql_connection
    }
}