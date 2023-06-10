use reqwest::{Client, Error};
use scraper::{Html, Selector};

pub struct Parser {
    requester: Client,
}

impl Parser {
    pub fn new() -> Self {
        let requester = Client::new();
        Self { requester }
    }

    pub fn get_word_defenition(word: &str) {
        let word = word.to_lowercase();

        let url = format!("https://www.oxfordlearnersdictionaries.com/definition/english/{word}");

        let html = Html::parse_document(get_page(url).unwrap().as_str());

        let selector = Selector::parse(r#"span"#).unwrap();
        for element in html.select(&selector) {
            if element.value().attr("class") ==  Some("def") {
                println!("found!");
            }
        }
    }
}

fn get_page(url: String) -> Result<String, Error> {
    let html = reqwest::blocking::get(url)?;
    html.text()
}
