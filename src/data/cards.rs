use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardJson {
    id: String,
    pub name: String,
    pub top: u8,
    pub right: u8,
    pub bottom: u8,
    pub left: u8,
    // we must rename because `type` keyword is reserved
    #[serde(rename = "type")]
    pub tribe: u16,
    sort: u16,
    order: u16,
}

#[derive(Debug)]
pub struct CardData {
    cards: Vec<CardJson>,
}

impl CardData {
    pub fn by_id(&self, id: &str) -> Option<CardJson> {
        for card in self.cards.iter() {
            if card.id == id {
                return Some(card.clone());
            }
        }

        return None;
    }

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
