use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct RuleJson {
    id: String,
    name: String,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RuleData {
    rules: Vec<RuleJson>,
}

impl RuleData {
    pub fn read() -> RuleData {
        let file = fs::File::open(JSON_PATH).expect("file should open read only");
        let json: serde_json::Value =
            serde_json::from_reader(file).expect("file should be proper JSON");

        let rule_list = json.as_array().unwrap();

        let mut rules: Vec<RuleJson> = vec![];

        for rule_value in rule_list {
            let rule_json = RuleJson::deserialize(rule_value).unwrap();
            rules.push(rule_json);
        }

        let data = RuleData { rules };

        return data;
    }
}

const JSON_PATH: &str = "./js/dist/rules.json";
