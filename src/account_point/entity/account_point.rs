use std::os::raw::c_uint;
use bcrypt::{BcryptError};
use diesel::{Expression, Insertable, Queryable, table};
use diesel::mysql::data_types::MysqlTime;

#[derive(Queryable, Insertable, Debug)]
#[table_name = "account_points"]
pub struct AccountPoint {
    #[column_name = "account_id"]
    pub account_id: i32,
    pub gold: i32,
    pub event_check: i32,
    pub free_gacha_check: MysqlTime,
}

table! {
    account_points (account_id) {
        account_id -> Integer,
        gold -> Integer,
        event_check -> Integer,
        free_gacha_check -> Date
    }
}

impl AccountPoint {
    pub fn new(account_id: i32, gold: i32, event_check: i32, free_gacha_check: MysqlTime) -> Result<Self, BcryptError> {

        Ok(AccountPoint {
            account_id,
            gold,
            event_check,
            free_gacha_check,
        })
    }

    pub fn account_id(&self) -> i32 { self.account_id }

    pub fn gold(&self) -> i32 { self.gold }
    pub fn event_check(&self) -> i32 { self.event_check }
    pub fn get_free_gacha_year(&self) -> c_uint { self.free_gacha_check.year }
    pub fn get_free_gacha_month(&self) -> c_uint { self.free_gacha_check.month }
    pub fn get_free_gacha_day(&self) -> c_uint { self.free_gacha_check.day }
    pub fn get_free_gacha_date(&self) -> Vec<c_uint> { vec![self.free_gacha_check.year, self.free_gacha_check.month, self.free_gacha_check.day]}
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