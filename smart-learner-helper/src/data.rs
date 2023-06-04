use std::{
    fs::OpenOptions,
    io::{self, Write},
    path::Path,
};

use smart_learner_core::deck::Deck;

pub struct DeckFromFile {
    pub value: Deck,
    path: String,
}

impl Drop for DeckFromFile {
    fn drop(&mut self) {
        let data = serde_json::to_string(&self.value).unwrap();
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&self.path)
            .unwrap();
        let _ = file.write(data.as_bytes());
    }
}

pub fn fetch_decks(path: &Path) -> Vec<DeckFromFile> {
    let mut decks = Vec::new();
    for file in path.read_dir().unwrap() {
        let path = file.unwrap();
        if path.file_name().to_str().unwrap().contains(".sdeck") {
            let path = &path.path();
            decks.push(DeckFromFile {
                value: load_deck(path),
                path: path.to_str().unwrap().to_string(),
            });
        }
    }
    decks
}

fn load_deck(path: &Path) -> Deck {
    let file = OpenOptions::new().read(true).open(path).unwrap();
    let data = io::read_to_string(file).unwrap();
    serde_json::from_str(&data).unwrap()
}
