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
    id: String,
    name: String,
    guaranteed: Vec<Card>,
    variable: Vec<Card>,
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
            if npc.name.contains(&normalized_search) {
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

        for rule_value in npc_list {
            let npc_json = NpcJson::deserialize(rule_value).unwrap();

            // use card_data and rule_data to hydrate the npc as we read it in

            let mut guaranteed = vec![];
            for card_id in npc_json.guaranteed {
                if let Some(card_json) = card_data.by_id(&card_id) {
                    // build Card instance with guarantee flag
                    // Card::computer(...)

                    let name = &format!("C{}", guaranteed.len());

                    guaranteed.push(Card::computer_guaranteed(
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

            let variable = vec![];

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
                guaranteed,
                variable,
                rules,
            });
        }

        let data = NpcData { npcs };

        return data;
    }
}

const JSON_PATH: &str = "./js/dist/npcs.json";
