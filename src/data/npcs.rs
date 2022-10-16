use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct NpcJson {
    id: String,
    name: String,
    guaranteed: Vec<String>,
    variable: Vec<String>,
    rules: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NpcData {
    npcs: Vec<NpcJson>,
}

const JSON_PATH: &str = "./js/dist/npcs.json";

pub fn parse_npcs() -> NpcData {
    let file = fs::File::open(JSON_PATH).expect("file should open read only");
    let json: serde_json::Value =
        serde_json::from_reader(file).expect("file should be proper JSON");

    let npc_list = json.as_array().unwrap();

    let mut npcs: Vec<NpcJson> = vec![];

    for rule_value in npc_list {
        let rule_json = NpcJson::deserialize(rule_value).unwrap();
        npcs.push(rule_json);
    }

    let data = NpcData { npcs };

    return data;
}
