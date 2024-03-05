use crate::account_deck_card::entity::deck_card::DeckCard;

#[derive(Debug)]
pub struct AccountDeckCardList {
    account_deck_card_list: Vec<DeckCard>,
}

impl AccountDeckCardList {
    pub fn new() -> AccountDeckCardList {AccountDeckCardList { account_deck_card_list: Vec::new() }
    }

    pub fn add_card(&mut self, card: DeckCard) {
        self.account_deck_card_list.push(card);
    }

    pub fn get_all_card_list(&self) -> &Vec<DeckCard> {
        &self.account_deck_card_list
    }

    pub fn get_all_card_list_mut(&mut self) -> &mut Vec<DeckCard> {
        &mut self.account_deck_card_list
    }

    pub fn set_card_list(&mut self, card_list: Vec<DeckCard>) {
        self.account_deck_card_list = card_list;
    }
}