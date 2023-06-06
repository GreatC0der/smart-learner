use std::path::Path;

use smart_learner_core::{card::Card, deck::Deck, field::Field, result::Result};

use crate::{
    config::Config,
    data::{self, DeckFromFile},
};

pub struct App {
    pub config: Config,
    pub decks: Vec<DeckFromFile>,
    pub current_deck: usize,
    current_card: Option<usize>,
    pub card_front: String,
    pub card_back: String,
    pub search_text: String,
    pub back_search: bool,
}

impl App {
    pub fn new() -> Self {
        let config: Config = confy::load("smart-learner", None).unwrap();
        let decks = data::fetch_decks(&Path::new(&config.folder_path));
        Self {
            config,
            decks,
            current_deck: 0,
            current_card: None,
            card_front: String::new(),
            card_back: String::new(),
            search_text: String::new(),
            back_search: false,
        }
    }

    pub fn new_deck(&mut self, deck_name: String) {
        let folder_path = Path::new(&self.config.folder_path);
        let path = folder_path.join(Path::new(&deck_name));
        self.decks.push(DeckFromFile {
            value: Deck::new(deck_name),
            path: path.as_path().to_str().unwrap().to_string() + ".sdeck",
        });
    }

    pub fn get_card_for_revision(&mut self) -> bool {
        self.current_card = match self.current_card {
            Some(result) => {
                if self.decks[self.current_deck].value.cards[result].current_repeat_in == 0 {
                    Some(result)
                } else {
                    self.decks[self.current_deck].value.due_card()
                }
            }
            None => self.decks[self.current_deck].value.due_card(),
        };

        if self.current_card.is_some() {
            self.change_card(self.current_card.unwrap());
            true
        } else {
            false
        }
    }

    pub fn get_answer(&self) -> String {
        if self.current_card.is_some() {
            self.decks[self.current_deck].value.cards[self.current_card.unwrap()]
                .back
                .text
                .clone()
        } else {
            "".to_string()
        }
    }

    pub fn get_question(&self) -> String {
        if self.current_card.is_some() {
            self.decks[self.current_deck].value.cards[self.current_card.unwrap()]
                .front
                .text
                .clone()
        } else {
            "".to_string()
        }
    }

    pub fn current_deck_name(&self) -> String {
        if self.decks.len() > self.current_deck {
            self.decks[self.current_deck].value.name.clone()
        } else {
            "No decks".to_string()
        }
    }

    pub fn create_card(&mut self) {
        self.decks[self.current_deck].value.cards.push(Card::new(
            Field {
                text: "".to_string(),
            },
            Field {
                text: "".to_string(),
            },
        ));
        self.change_card(self.decks[self.current_deck].value.cards.len() - 1);
    }

    pub fn edit_card(&mut self) {
        self.decks[self.current_deck].value.cards[self.current_card.unwrap()] = Card::new(
            Field {
                text: self.card_front.clone(),
            },
            Field {
                text: self.card_back.clone(),
            },
        );
    }

    pub fn search(&mut self) -> Vec<(usize, String)> {
        if self.decks.len() == 0 {
            return Vec::new();
        }

        self.decks[self.current_deck]
            .value
            .search(self.back_search, self.search_text.clone())
    }

    pub fn change_card(&mut self, card_index: usize) {
        self.current_card = Some(card_index);
        let card = &self.decks[self.current_deck].value.cards[self.current_card.unwrap()];
        self.card_front = card.front.text.clone();
        self.card_back = card.back.text.clone();
    }

    pub fn card_revised(&mut self, result: Result) {
        self.decks[self.current_deck].value.cards[self.current_card.unwrap()].review(result);
    }
}
