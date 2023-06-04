use crate::card::Card;
use crate::date::Date;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Deck {
    pub name: String,
    pub cards: Vec<Card>,
    pub last_update: Date, // day, month, year
}

impl Deck {
    pub fn new(name: String) -> Self {
        Deck {
            name,
            cards: Vec::new(),
            last_update: Date::current(),
        }
    }
    pub fn update(&mut self) {
        let current_date = Date::current();
        if current_date <= self.last_update {
            return;
        }

        let days_since_last_update = self.last_update.difference(&current_date);

        for card_index in 0..self.cards.len() {
            if self.cards[card_index].current_repeat_in > 0 {
                self.cards[card_index].current_repeat_in -= days_since_last_update;
            }
        }

        self.last_update = current_date;
    }
}
