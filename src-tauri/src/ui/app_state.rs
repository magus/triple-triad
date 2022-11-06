use std::fs;
use std::sync::Mutex;
use tauri::App;

use crate::data;
use crate::game::Game;

pub struct AppState {
    // use interior mutability since the instance of AppState cannot change
    // that means we Mutex fields so we can change their values
    // this gives us a way to edit shared game state
    // accessible via commands (invoke) and also rust app handles
    pub status: Mutex<String>,
    pub game: Mutex<Game>,
    pub setup_game: Mutex<Game>,

    // shared instances, created once and reused
    pub rule_data: Mutex<Option<data::RuleData>>,
    pub card_data: Mutex<Option<data::CardData>>,
    pub npc_data: Mutex<Option<data::NpcData>>,
}

// serialized json, subset of AppState
#[derive(Clone, serde::Serialize)]
pub struct AppStateJson {
    status: String,
    turn_is_player: bool,
    game: Game,
}

impl AppState {
    pub fn json(&self) -> AppStateJson {
        let game = self.game.lock().unwrap().clone();
        let status = self.status.lock().unwrap().clone();

        let turn_is_player = game.turn_is_player();

        return AppStateJson {
            status,
            turn_is_player,
            game,
        };
    }

    pub fn set_status(&self, value: &str) {
        let mut status = self.status.lock().unwrap();
        *status = value.to_string();
    }

    pub fn set_game(&self, game: Game) {
        let mut game_mutex = self.game.lock().unwrap();
        *game_mutex = game;
    }

    pub fn set_setup_game(&self, game: Game) {
        let mut setup_game_mutex = self.setup_game.lock().unwrap();
        *setup_game_mutex = game;
    }

    pub fn init_data(&self, app: &App) {
        let rule_data = data::RuleData::read(load_resource(app, "../data/game/rules.json"));
        let card_data = data::CardData::read(load_resource(app, "../data/game/cards.json"));
        let npc_data = data::NpcData::read(
            load_resource(app, "../data/game/npcs.json"),
            &card_data,
            &rule_data,
        );

        let mut rule_data_mutex = self.rule_data.lock().unwrap();
        *rule_data_mutex = Some(rule_data);

        let mut card_data_mutex = self.card_data.lock().unwrap();
        *card_data_mutex = Some(card_data);

        let mut npc_data_mutex = self.npc_data.lock().unwrap();
        *npc_data_mutex = Some(npc_data);
    }

    pub fn new() -> AppState {
        AppState {
            status: Mutex::new("setup".into()),
            game: Mutex::new(Game::new()),
            setup_game: Mutex::new(Game::new()),

            rule_data: Mutex::new(None),
            card_data: Mutex::new(None),
            npc_data: Mutex::new(None),
        }
    }
}

fn load_resource(app: &App, path: &str) -> Option<fs::File> {
    if let Ok(file) = std::fs::File::open(
        app.path_resolver()
            .resolve_resource(path)
            .expect("failed to resolve resource"),
    ) {
        println!("loading from resource path [{path}]");
        return Some(file);
    }

    println!("fallback, could not load [{path}]");
    return None;
}
