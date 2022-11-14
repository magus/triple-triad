use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleJson {
    pub id: String,
    pub name: String,
    description: String,
}

impl RuleJson {
    pub fn is_roulette(&self) -> bool {
        self.id == "1"
    }

    pub fn is_all_open(&self) -> bool {
        self.id == "2"
    }

    pub fn is_three_open(&self) -> bool {
        self.id == "3"
    }

    pub fn is_same(&self) -> bool {
        self.id == "4"
    }

    pub fn is_sudden_death(&self) -> bool {
        self.id == "5"
    }

    pub fn is_plus(&self) -> bool {
        self.id == "6"
    }

    pub fn is_random(&self) -> bool {
        self.id == "7"
    }

    pub fn is_order(&self) -> bool {
        self.id == "8"
    }

    pub fn is_chaos(&self) -> bool {
        self.id == "9"
    }

    pub fn is_reverse(&self) -> bool {
        self.id == "10"
    }

    pub fn is_fallen_ace(&self) -> bool {
        self.id == "11"
    }

    pub fn is_ascension(&self) -> bool {
        self.id == "12"
    }

    pub fn is_descension(&self) -> bool {
        self.id == "13"
    }

    pub fn is_swap(&self) -> bool {
        self.id == "14"
    }

    pub fn is_draft(&self) -> bool {
        self.id == "15"
    }
}

#[derive(Debug)]
pub struct RuleData {
    pub rules: Vec<RuleJson>,
}

impl RuleData {
    pub fn by_id(&self, id: &str) -> Option<RuleJson> {
        for rule in self.rules.iter() {
            if rule.id == id {
                return Some(rule.clone());
            }
        }

        return None;
    }

    pub fn read(maybe_file: Option<fs::File>) -> RuleData {
        // use provided file or fallback to local fs path
        let file = if let Some(file_arg) = maybe_file {
            file_arg
        } else {
            fs::File::open(JSON_PATH).expect("file should open read only")
        };

        let json: serde_json::Value =
            serde_json::from_reader(file).expect("file should be proper JSON");

        let rule_list = json.as_array().unwrap();

        let mut rules = vec![];

        for json_value in rule_list {
            let rule_json = RuleJson::deserialize(json_value).unwrap();

            rules.push(rule_json);
        }

        return RuleData { rules };
    }
}

const JSON_PATH: &str = "./data/rules.json";
