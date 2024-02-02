use bcrypt::{BcryptError};
use diesel::{Insertable, Queryable, table};

#[derive(Queryable, Insertable, Debug)]
#[table_name = "account_points"]
pub struct AccountPoint {
    #[column_name = "account_id"]
    pub account_id: i32,
    pub gold: i32,
}

table! {
    account_points (account_id) {
        account_id -> Integer,
        gold -> Integer,
    }
}

impl AccountPoint {
    pub fn new(account_id: i32, gold: i32) -> Result<Self, BcryptError> {

        Ok(AccountPoint {
            account_id,
            gold,
        })
    }

    pub fn account_id(&self) -> i32 { self.account_id }

    pub fn gold(&self) -> i32 { self.gold }
}

impl std::fmt::Display for AccountPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Account_points {{ account_id: {}, gold: {} }}",
            self.account_id, self.gold
        )
    }
}