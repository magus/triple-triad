use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardJson {
    pub id: String,
    pub name: String,
    pub top: u8,
    pub right: u8,
    pub bottom: u8,
    pub left: u8,
    pub tribe: u8,
    sort: u16,
    order: u16,
}

#[derive(Debug)]
pub struct CardData {
    cards_list: Vec<CardJson>,
    cards_map: HashMap<String, CardJson>,
}

impl CardData {
    pub fn by_id(&self, id: &str) -> Option<CardJson> {
        if let Some(card) = self.cards_map.get(id) {
            return Some(card.clone());
        }

        return None;
    }

    pub fn search(&self, search: &str) -> Vec<&CardJson> {
        let normalized_search = search.to_lowercase();
        let mut card_list = vec![];

        for card in self.cards_list.iter() {
            let normalized_name = card.name.to_lowercase();
            if normalized_name.contains(&normalized_search) {
                card_list.push(card);
            }
        }

        return card_list;
    }

    pub fn read() -> CardData {
        let file = fs::File::open(JSON_PATH).expect("file should open read only");
        let json: serde_json::Value =
            serde_json::from_reader(file).expect("file should be proper JSON");

        let card_list = json.as_array().unwrap();

        let mut cards_list: Vec<CardJson> = vec![];
        let mut cards_map: HashMap<String, CardJson> = HashMap::new();

        for json_value in card_list {
            let card_json = CardJson::deserialize(json_value).unwrap();

            cards_list.push(card_json.clone());
            cards_map.insert(card_json.id.clone(), card_json.clone());
        }

        return CardData {
            cards_list,
            cards_map,
        };
    }
}

const JSON_PATH: &str = "../data/game//cards.json";
