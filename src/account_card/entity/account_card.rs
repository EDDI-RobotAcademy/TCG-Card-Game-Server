use bcrypt::BcryptError;
use diesel::{Insertable, Queryable, table};

#[derive(Queryable, Insertable, Debug)]
#[table_name = "account_cards"]
pub struct AccountCard {
    #[column_name = "account_id"]
    pub account_id: i32,
    pub card_id: i32,
    pub card_count: i32,
}

table! {
    account_cards (card_id) {
        account_id -> Integer,
        card_id -> Integer,
        card_count -> Integer,
    }
}

impl AccountCard {
    pub fn new(account_id: i32, card_id: i32, card_count: i32) -> Result<Self, BcryptError> {

        Ok(AccountCard {
            account_id,
            card_id,
            card_count,
        })
    }
    pub fn account_id(&self) -> i32 { self.account_id }
    pub fn card_id(&self) -> i32 { self.card_id }
    pub fn card_counts(&self) -> i32 { self.card_count }
}


impl std::fmt::Display for AccountCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AccountCard {{ account_id: {}, card_id: {}, card_counts: {} }}",
            self.account_id, self.card_id, self.card_count
        )
    }
}