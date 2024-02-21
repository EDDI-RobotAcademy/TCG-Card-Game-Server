use bcrypt::{BcryptError};
use chrono::NaiveDate;
use diesel::{Expression, Insertable, Queryable, table};
use diesel::expression::AsExpression;

#[derive(Queryable, Insertable, Debug)]
#[table_name = "account_points"]
pub struct AccountPoint {
    #[column_name = "account_id"]
    pub account_id: i32,
    pub gold: i32,
    pub event_check: i32,
}

table! {
    account_points (account_id) {
        account_id -> Integer,
        gold -> Integer,
        event_check -> Integer,
    }
}

impl AccountPoint {
    pub fn new(account_id: i32, gold: i32, event_check: i32) -> Result<Self, BcryptError> {

        Ok(AccountPoint {
            account_id,
            gold,
            event_check,
        })
    }

    pub fn account_id(&self) -> i32 { self.account_id }

    pub fn gold(&self) -> i32 { self.gold }
    pub fn event_check(&self) -> i32 { self.event_check }
}
//
// impl std::fmt::Display for AccountPoint {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "Account_points {{ account_id: {}, gold: {} }}",
//             self.account_id, self.gold
//         )
//     }
// }