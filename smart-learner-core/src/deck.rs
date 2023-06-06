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

    pub fn due_card(&self) -> Option<usize> {
        let mut result = None;
        for card in self.cards.iter().enumerate() {
            if card.1.current_repeat_in == 0 {
                result = Some(card.0);
            }
        }
        result
    }

    pub fn search(&self, back_search: bool, search_text: String) -> Vec<(usize, String)>{
        let mut result = Vec::new();
        for (card_index, card) in self.cards.iter().enumerate() {
            if back_search {
                if card.back.text.contains(&search_text) {
                    result.push((card_index, card.back.text.clone()))
                }
            } else {
                if card.front.text.contains(&search_text) {
                    result.push((card_index, card.front.text.clone()))
                }
            }
        }
        result
    }
}
