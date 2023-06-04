use std::path::Path;

use smart_learner_core::deck::Deck;

use crate::{
    config::Config,
    data::{self, DeckFromFile},
};
pub struct App {
    pub config: Config,
    pub decks: Vec<DeckFromFile>,
}

impl App {
    pub fn new() -> Self {
        let config: Config = confy::load("smart-learner", None).unwrap();
        let decks = data::fetch_decks(&Path::new(&config.folder_path));
        Self { config, decks }
    }

    pub fn new_deck(&mut self, deck_name: String) {
        let folder_path = Path::new(&self.config.folder_path);
        let path = folder_path.join(Path::new(&deck_name));
        self.decks.push(DeckFromFile {
            value: Deck::new(deck_name),
            path: path.as_path().to_str().unwrap().to_string() + ".sdeck",
        });
    }
}
