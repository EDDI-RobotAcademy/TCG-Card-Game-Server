use bcrypt::{BcryptError};
use diesel::{Insertable, Queryable, table};

#[derive(Queryable, Insertable, Debug)]
#[table_name = "account_decks"]
pub struct AccountDeck {
    #[column_name = "deck_id"]
    pub deck_id: i32,
    pub account_id: i32,
    pub deck_name: String,
}

table! {
    account_decks (deck_id) {
        deck_id -> Integer,
        account_id -> Integer,
        deck_name -> Text,
    }
}

impl AccountDeck {
    pub fn new(account_id: i32, deck_name: &str) -> Result<Self, BcryptError> {

        Ok(AccountDeck {
            deck_id: 0,
            account_id,
            deck_name: deck_name.to_string(),
        })
    }

    pub fn deck_id(&self) -> i32 { self.deck_id }

    pub fn account_id(&self) -> i32 { self.account_id }

    pub fn deck_name(&self) -> &str { &self.deck_name }
}

impl std::fmt::Display for AccountDeck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AccountDeck {{ deck_id: {}, account_id: {}, deck_name: {} }}",
            self.deck_id, self.account_id, self.deck_name
        )
    }
}