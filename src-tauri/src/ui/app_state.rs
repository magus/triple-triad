use std::sync::Mutex;

use crate::data;
use crate::game::Game;

pub struct AppState {
    // use interior mutability since the instance of AppState cannot change
    // that means we Mutex fields so we can change their values
    // this gives us a way to edit shared game state
    // accessible via commands (invoke) and also rust app handles
    pub status: Mutex<String>,
    pub game: Mutex<Game>,

    // shared instances, created once and reused
    pub rule_data: data::RuleData,
    pub card_data: data::CardData,
    pub npc_data: data::NpcData,
}

// serialized json, subset of AppState
#[derive(Clone, serde::Serialize)]
pub struct AppStateJson {
    status: String,
    game: Game,
}

impl AppState {
    pub fn json(&self) -> AppStateJson {
        let json = AppStateJson {
            status: self.status.lock().unwrap().clone(),
            game: self.game.lock().unwrap().clone(),
        };

        return json;
    }

    pub fn set_status(&self, value: &str) {
        let mut status = self.status.lock().unwrap();
        *status = value.to_string();
    }

    pub fn set_game(&self, game: Game) {
        let mut game_mutex = self.game.lock().unwrap();
        *game_mutex = game;
    }

    pub fn new() -> AppState {
        let rule_data = data::RuleData::read();
        let card_data = data::CardData::read();
        let npc_data = data::NpcData::read(&card_data, &rule_data);

        AppState {
            status: Mutex::new("setup".into()),
            game: Mutex::new(Game::new()),

            rule_data,
            card_data,
            npc_data,
        }
    }
}
