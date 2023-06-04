use std::path::Path;

use crate::{config::Config, data::{self, DeckFromFile}};
pub struct App {
    pub config: Config,
    pub decks: Vec<DeckFromFile>
}

impl App {
    pub fn new() -> Self {
        let config: Config = confy::load("smart-learner", None).unwrap();
        let decks = data::fetch_decks(&Path::new(&config.folder_path));
        Self { config, decks}
    }
}