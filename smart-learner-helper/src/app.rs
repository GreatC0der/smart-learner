use std::path::Path;

use smart_learner_core::{card::Card, deck::Deck, field::Field};

use crate::{
    config::Config,
    data::{self, DeckFromFile},
};

pub struct App {
    pub config: Config,
    pub decks: Vec<DeckFromFile>,
    pub current_deck: usize,
    pub current_card: Option<usize>,
    pub new_card_front: String,
    pub new_card_back: String,
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
            new_card_front: String::new(),
            new_card_back: String::new(),
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

    pub fn get_front_for_revision(&mut self) -> Option<String> {
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

        match self.current_card {
            Some(result) => Some(
                self.decks[self.current_deck].value.cards[result]
                    .front
                    .text
                    .clone(),
            ),
            None => None,
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
                text: self.new_card_front.clone(),
            },
            Field {
                text: self.new_card_back.clone(),
            },
        ));
    }
}
