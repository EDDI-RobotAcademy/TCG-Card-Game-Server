use bcrypt::{BcryptError};
use diesel::{Insertable, Queryable, table};

#[derive(Queryable, Insertable, Debug)]
#[table_name = "account_points"]
pub struct AccountId {
    #[column_name = "account_id"]
    pub account_id: i32,
}

table! {
    account_points (account_id) {
        account_id -> Integer,
    }
}

impl AccountId {
    pub fn new(account_id: i32) -> Result<Self, BcryptError> {

        Ok(AccountId {
            account_id,
        })
    }

    pub fn account_id(&self) -> i32 { self.account_id }

}