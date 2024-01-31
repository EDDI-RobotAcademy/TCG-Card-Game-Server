use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

#[derive(Queryable, Debug)]
struct AccountCard {
    account_id: i32,
    card_id: i32,
    card_count: i32,
}

table! {
    account_cards (account_id, card_id) {
        account_id -> Integer,
        card_id -> Integer,
        card_count -> Integer,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::env::env_detector::EnvDetector;

    #[test]
    fn test_real_database_connection() {
        use crate::account::entity::account::accounts::dsl::*;

        println!("AccountRepositoryImpl: save()");

        let database_url = EnvDetector::get_mysql_url().expect("DATABASE_URL이 설정되어 있어야 합니다.");
        let mut connection = MysqlConnection::establish(&database_url)
            .expect("Failed to establish a new connection");

        let result = account_cards::table
            .filter(account_cards::account_id.eq(1).and(account_cards::card_count.eq(3)))
            .load::<AccountCard>(&mut connection)
            .expect("Error loading data");

        println!("Result: {:?}", result);
    }
}
