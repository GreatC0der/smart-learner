use std::{path::Path, fs::read_to_string};

use smart_learner_core::deck::Deck;

pub fn fetch_decks(path: &Path) {
    let mut decks = Vec::new();
    for file in path.read_dir().unwrap() {
        let path = file.unwrap();
        if path.file_name().to_str().unwrap().contains(".sdeck") {
            decks.push(load_deck(&path.path()));
        }
    }
}

fn load_deck(path: &Path) -> Deck{
    let data = read_to_string(path).unwrap();
    serde_json::from_str(&data).unwrap()
}