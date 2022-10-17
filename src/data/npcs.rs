use serde::{Deserialize, Serialize};
use std::fs;

use super::{rules::RuleJson, CardData, RuleData};
use crate::card::Card;

#[derive(Debug, Serialize, Deserialize)]
pub struct NpcJson {
    id: String,
    name: String,
    guaranteed: Vec<String>,
    variable: Vec<String>,
    rules: Vec<String>,
}

#[derive(Debug)]
pub struct Npc {
    #[allow(dead_code)]
    id: String,
    name: String,
    pub cards: Vec<Card>,
    #[allow(dead_code)]
    rules: Vec<RuleJson>,
}

#[derive(Debug)]
pub struct NpcData {
    npcs: Vec<Npc>,
}

impl NpcData {
    pub fn find_all_npc(&self, search: &str) -> Vec<&Npc> {
        let normalized_search = search.to_lowercase();
        let mut npc_list = vec![];

        for npc in self.npcs.iter() {
            let normalized_name = npc.name.to_lowercase();
            if normalized_name.contains(&normalized_search) {
                npc_list.push(npc);
            }
        }

        return npc_list;
    }

    pub fn read(card_data: &CardData, rule_data: &RuleData) -> NpcData {
        let file = fs::File::open(JSON_PATH).expect("file should open read only");
        let json: serde_json::Value =
            serde_json::from_reader(file).expect("file should be proper JSON");

        let npc_list = json.as_array().unwrap();

        let mut npcs = vec![];

        for json_value in npc_list {
            let npc_json = NpcJson::deserialize(json_value).unwrap();

            // use card_data and rule_data to hydrate the npc as we read it in

            let mut cards = vec![];

            for card_id in npc_json.guaranteed {
                if let Some(card_json) = card_data.by_id(&card_id) {
                    let name = card_name(cards.len());

                    cards.push(Card::computer_guaranteed(
                        name,
                        card_json.top,
                        card_json.right,
                        card_json.bottom,
                        card_json.left,
                    ));
                } else {
                    panic!("unexpected card_id [{card_id}]");
                }
            }

            for card_id in npc_json.variable {
                if let Some(card_json) = card_data.by_id(&card_id) {
                    let name = card_name(cards.len());

                    cards.push(Card::computer(
                        name,
                        card_json.top,
                        card_json.right,
                        card_json.bottom,
                        card_json.left,
                    ));
                } else {
                    panic!("unexpected card_id [{card_id}]");
                }
            }

            let mut rules = vec![];

            for rule_id in npc_json.rules {
                if let Some(rule) = rule_data.by_id(&rule_id) {
                    rules.push(rule);
                } else {
                    panic!("unexpected rule_id [{rule_id}]");
                }
            }

            npcs.push(Npc {
                id: npc_json.id,
                name: npc_json.name,
                cards,
                rules,
            });
        }

        let data = NpcData { npcs };

        return data;
    }
}

const JSON_PATH: &str = "./js/dist/npcs.json";

fn card_name(size: usize) -> &'static str {
    return match size {
        0 => "C0",
        1 => "C1",
        2 => "C2",
        3 => "C3",
        4 => "C4",
        5 => "C5",
        6 => "C6",
        7 => "C7",
        8 => "C8",
        9 => "C9",
        _ => panic!("unexpected number of npc cards [{}]", size),
    };
}
