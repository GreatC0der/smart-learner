use std::path::Path;

use crate::{config::Config, files};
pub struct App {
    pub config: Config,
}

impl App {
    pub fn new() -> Self {
        let config: Config = confy::load("smart-learner", None).unwrap();
        files::fetch_decks(&Path::new(&config.folder_path));
        Self { config }
    }
}