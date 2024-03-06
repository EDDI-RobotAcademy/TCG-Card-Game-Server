use crate::account_card::entity::card::Card;
#[derive(Debug)]
pub struct AccountCardList {
    account_card_list: Vec<Card>,
}

impl AccountCardList {
    pub fn new() -> AccountCardList {AccountCardList { account_card_list: Vec::new() }
    }

    pub fn add_card(&mut self, card: Card) {
        self.account_card_list.push(card);
    }

    pub fn get_all_card_list(&self) -> &Vec<Card> {
        &self.account_card_list
    }

    pub fn get_all_card_list_mut(&mut self) -> &mut Vec<Card> {
        &mut self.account_card_list
    }

    pub fn set_card_list(&mut self, card_list: Vec<Card>) {
        self.account_card_list = card_list;
    }
}