use serde::{Deserialize, Serialize};
use std::fs;

use crate::data;

#[derive(Debug, Serialize, Deserialize)]
pub struct NpcJson {
    id: String,
    name: String,
    guaranteed: Vec<String>,
    variable: Vec<String>,
    rules: Vec<String>,
}

#[derive(Debug)]
pub struct NpcData {
    npcs: Vec<NpcJson>,
}

impl NpcData {
    pub fn find_all_npc(&self, search: &str) -> Vec<&NpcJson> {
        let normalized_search = search.to_lowercase();
        let mut npc_list = vec![];

        for npc in self.npcs.iter() {
            if npc.name.contains(&normalized_search) {
                npc_list.push(npc);
            }
        }

        return npc_list;
    }

    pub fn read() -> NpcData {
        let file = fs::File::open(JSON_PATH).expect("file should open read only");
        let json: serde_json::Value =
            serde_json::from_reader(file).expect("file should be proper JSON");

        let npc_list = json.as_array().unwrap();

        let mut npcs: Vec<NpcJson> = vec![];

        for rule_value in npc_list {
            let rule_json = NpcJson::deserialize(rule_value).unwrap();
            npcs.push(rule_json);
        }

        let data = NpcData { npcs, card_data };

        return data;
    }
}

const JSON_PATH: &str = "./js/dist/npcs.json";
