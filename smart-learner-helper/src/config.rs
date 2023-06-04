use serde_derive::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub folder_path: String,
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            folder_path: env::current_dir().unwrap().to_str().unwrap().to_string(),
        }
    }
}

impl Drop for Config {
    fn drop(&mut self) {
        confy::store("smart-learner", None, self).unwrap();
    }
}