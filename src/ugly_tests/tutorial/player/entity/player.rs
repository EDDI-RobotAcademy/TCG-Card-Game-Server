use futures::SinkExt;

#[derive(Debug)]
pub struct Player {
    life_point: i32,
    turn_number: i32,
    field_energy: i32,
    deck_card_list: Vec<i32>,
    hand_card_list: Vec<i32>,
    field_card_list: Vec<i32>,
    grave_card_list: Vec<i32>,
    lost_card_list: Vec<i32>
}

impl Player {
    pub fn new() -> Self {
        Player {
            life_point: 40,
            turn_number: 0,
            field_energy: 0,
            deck_card_list: Vec::new(),
            hand_card_list: Vec::new(),
            field_card_list: Vec::new(),
            grave_card_list: Vec::new(),
            lost_card_list: Vec::new()
        }
    }
    pub fn set_life_point(&mut self, heal_or_damage: i32) {
        self.life_point += heal_or_damage
    }
    pub fn get_life_point(&self) -> i32 {
        self.life_point
    }
    pub fn add_turn_number(&mut self, times: i32) {
        self.turn_number += times;
    }
    pub fn get_turn_number(&self) -> i32 {
        self.turn_number
    }
    pub fn add_field_energy(&mut self, energy: i32) {
        self.field_energy += energy;
    }
    pub fn use_field_energy_to_card(&mut self, amount_of_energy: i32) {
        if self.field_energy - amount_of_energy < 0 {
            self.field_energy = 0
        } else { self.field_energy -= amount_of_energy }
    }
    pub fn get_field_energy(&self) -> i32 {
        self.field_energy
    }
    pub fn append_cards_to_deck(&mut self, cards: Vec<i32>) {
        for card in cards {
            self.deck_card_list.push(card)
        }
    }
    pub fn draw_cards_from_deck(&mut self, how_many_cards: usize) {
        let drawn_cards = self.deck_card_list.iter().take(how_many_cards).cloned().collect();
        self.deck_card_list.drain(0..how_many_cards);
        self.append_to_hand(drawn_cards);
    }
    pub fn get_deck(&self) -> &Vec<i32> {
        &self.deck_card_list
    }
    pub fn append_to_hand(&mut self, cards: Vec<i32>) {
        for card in cards {
            self.hand_card_list.push(card)
        }
    }
    pub fn use_card_from_hand(&mut self, card: i32) {
        self.hand_card_list = self.hand_card_list.iter().filter(|x| x != &&card).cloned().collect();
        self.field_card_list.push(card);
    }
    pub fn get_hand(&self) -> &Vec<i32> {
        &self.hand_card_list
    }
    pub fn append_to_field(&mut self, cards: Vec<i32>) {
        for card in cards {
            self.field_card_list.push(card)
        }
    }
    pub fn get_field(&self) -> &Vec<i32> {
        &self.field_card_list
    }
    pub fn append_to_grave(&mut self, cards: Vec<i32>) {
        for card in cards {
            self.grave_card_list.push(card)
        }
    }
    pub fn get_grave(&self) -> &Vec<i32> {
        &self.grave_card_list
    }
    pub fn append_to_lost(&mut self, cards: Vec<i32>) {
        for card in cards {
            self.lost_card_list.push(card)
        }
    }
    pub fn get_lost(&self) -> &Vec<i32> {
        &self.lost_card_list
    }
}

