extern crate rand;

use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct BattleDeck {
    deck_num: i32,
    deck_card_list: Vec<i32>,

}

impl BattleDeck {


    pub fn new() -> BattleDeck {
        BattleDeck {
            deck_num: i32,
            deck_card_list: Vec::new(),
        }
    }
    pub fn deck_shuffle(&mut self)  {
        // let mut vec = &mut self.deck_card_list;
        let mut rng = thread_rng();

        // vec.shuffle(&mut rng);
        self.deck_card_list.shuffle(&mut rng);


    }

    pub fn drew_cards(&mut self, num: i32) -> Vec<i32> {
        let mut i = 1;
        let mut drew_cards = Vec::new();
        loop {
            if ( i > num){
                break;
            }

            let deck_pop = self.deck_card_list.pop().unwrap();
            drew_cards.push(deck_pop);
            i += 1;

        }
        return drew_cards;
    }
}