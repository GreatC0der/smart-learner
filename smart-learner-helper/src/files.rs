use std::path::Path;

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
    todo!()
}