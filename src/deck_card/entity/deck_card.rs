use bcrypt::{BcryptError};
use diesel::{Insertable, Queryable, table};
use diesel::sql_types::Integer;

#[derive(Queryable, Insertable, Debug)]
#[table_name = "deck_cards"]
pub struct DeckCard {
    #[column_name = "deck_id"]
    pub deck_id: i32,
    pub card_id: i32,
    pub card_count: i32,
}

table! {
    deck_cards (deck_id, card_id) {
        deck_id -> Integer,
        card_id -> Integer,
        card_count -> Integer,
    }
}

impl DeckCard {
    pub fn new(deck_id: i32, card_id: i32, card_count: i32) -> Result<Self, BcryptError> {
        Ok(DeckCard {
            deck_id,
            card_id,
            card_count
        })
    }
    pub fn deck_id(&self) -> i32 { self.deck_id }
    pub fn card_id(&self) -> i32 { self.card_id }
    pub fn card_count(&self) -> i32 { self.card_count }
}
