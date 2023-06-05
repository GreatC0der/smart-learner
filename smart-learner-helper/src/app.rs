use std::path::Path;

use smart_learner_core::{deck::Deck, card::Card};

use crate::{
    config::Config,
    data::{self, DeckFromFile},
};

pub struct App<'a>{
    pub config: Config,
    pub decks: Vec<DeckFromFile>,
    pub current_deck: usize,
    pub current_card: Option<&'a Card>,
}

impl App<'_> {
    pub fn new() -> Self {
        let config: Config = confy::load("smart-learner", None).unwrap();
        let decks = data::fetch_decks(&Path::new(&config.folder_path));
        Self {
            config,
            decks,
            current_deck: 0,
            current_card: None,
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
    
    pub fn get_front_for_revision(&mut self) -> Option<String>{
        let card = match self.current_card {
            Some(result) => {
                if result.current_repeat_in == 0 {
                    Some(result)
                } else {
                    self.decks[self.current_deck].value.due_card()
                }
            },
            None => {
                self.decks[self.current_deck].value.due_card()                
            }
        };
        
        match card {
            Some(result) => Some(result.front.text.clone()),
            None => None,
        }
    }
    
    pub fn get_answer(&self) -> String {
        self.current_card.unwrap().back.text.clone()
    }
    
    pub fn get_question(&self) -> String {
        self.current_card.unwrap().front.text.clone()
    }
}
