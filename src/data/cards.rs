use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct CardJson {
    id: String,
    name: String,
    top: u16,
    right: u16,
    bottom: u16,
    left: u16,
    // we must rename because `type` keyword is reserved
    #[serde(rename = "type")]
    tribe: u16,
    sort: u16,
    order: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardData {
    cards: Vec<CardJson>,
}

impl CardData {
    pub fn read() -> CardData {
        let file = fs::File::open(JSON_PATH).expect("file should open read only");
        let json: serde_json::Value =
            serde_json::from_reader(file).expect("file should be proper JSON");

        let card_list = json.as_array().unwrap();

        let mut cards: Vec<CardJson> = vec![];

        for rule_value in card_list {
            let card_json = CardJson::deserialize(rule_value).unwrap();
            cards.push(card_json);
        }

        let data = CardData { cards };

        return data;
    }
}

const JSON_PATH: &str = "./js/dist/cards.json";
