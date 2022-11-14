use std::fs;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::App;

use crate::data;
use crate::data::Npc;
use crate::game::{ExploreResult, Game, Rules};

use crate::ui::persist_data::PersistData;

pub struct AppState {
    // use interior mutability since the instance of AppState cannot change
    // that means we Mutex fields so we can change their values
    // this gives us a way to edit shared game state
    // accessible via commands (invoke) and also rust app handles
    pub status: Mutex<Option<String>>,
    pub game: Mutex<Game>,
    pub setup_game: Mutex<Game>,
    pub explore_result: Mutex<Option<ExploreResult>>,
    pub npc: Mutex<Option<Npc>>,
    pub pre_game: Mutex<Option<Rules>>,

    // shared instances, created once and reused
    pub rule_data: Mutex<Option<data::RuleData>>,
    pub card_data: Mutex<Option<data::CardData>>,
    pub npc_data: Mutex<Option<data::NpcData>>,
    pub persist_data: Mutex<Option<PersistData>>,
}

// serialized json, subset of AppState
#[derive(Clone, serde::Serialize)]
pub struct AppStateJson {
    // from app state
    game: Game,
    status: Option<String>,
    explore_result: Option<ExploreResult>,
    npc: Option<Npc>,
    persist_data: PersistData,

    // for client state
    now: u32,

    // via game internal methods
    turn_is_player: bool,
    is_ended: bool,
}

impl AppState {
    pub fn json(&self) -> AppStateJson {
        let game = self.game.lock().unwrap().clone();
        let status = self.status.lock().unwrap().clone();
        let explore_result = self.explore_result.lock().unwrap().clone();
        let npc = self.npc.lock().unwrap().clone();
        let persist_data = self.persist_data.lock().unwrap().clone().unwrap();

        let now = (SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            % u32::MAX as u128) as u32;

        let turn_is_player = game.turn_is_player();
        let is_ended = game.is_ended();

        return AppStateJson {
            game,
            status,
            explore_result,
            npc,
            persist_data,

            now,

            turn_is_player,
            is_ended,
        };
    }

    pub fn set_status(&self, value: Option<String>) {
        let mut status_mutex = self.status.lock().unwrap();
        *status_mutex = value;
    }

    pub fn set_game(&self, game: Game) {
        let mut game_mutex = self.game.lock().unwrap();
        *game_mutex = game;
    }

    pub fn set_setup_game(&self, game: Game) {
        let mut setup_game_mutex = self.setup_game.lock().unwrap();
        *setup_game_mutex = game;
    }

    pub fn set_explore_result(&self, explore_result: Option<ExploreResult>) {
        let mut explore_result_mutex = self.explore_result.lock().unwrap();
        *explore_result_mutex = explore_result;
    }

    pub fn set_npc(&self, npc: Option<Npc>) {
        let mut npc_mutex = self.npc.lock().unwrap();
        *npc_mutex = npc;
    }

    pub fn set_persist_data(&self, persist_data: Option<PersistData>) {
        let mut persist_data_mutex = self.persist_data.lock().unwrap();
        *persist_data_mutex = persist_data;
    }

    pub fn set_pre_game(&self, pre_game: Option<Rules>) {
        let mut pre_game_mutex = self.pre_game.lock().unwrap();
        *pre_game_mutex = pre_game;
    }

    pub fn init_data(&self, app: &App) {
        let rule_data = data::RuleData::read(load_resource(app, "./data/rules.json"));
        let card_data = data::CardData::read(load_resource(app, "./data/cards.json"));
        let npc_data = data::NpcData::read(
            load_resource(app, "./data/npcs.json"),
            &card_data,
            &rule_data,
        );

        let mut rule_data_mutex = self.rule_data.lock().unwrap();
        *rule_data_mutex = Some(rule_data);

        let mut card_data_mutex = self.card_data.lock().unwrap();
        *card_data_mutex = Some(card_data);

        let mut npc_data_mutex = self.npc_data.lock().unwrap();
        *npc_data_mutex = Some(npc_data);

        let mut persist_data_mutex = self.persist_data.lock().unwrap();
        let persist_data = PersistData::read(&app);
        *persist_data_mutex = Some(persist_data);
    }

    pub fn new() -> AppState {
        AppState {
            status: Mutex::new(Some("setup".into())),
            game: Mutex::new(Game::new()),
            setup_game: Mutex::new(Game::new()),
            explore_result: Mutex::new(None),
            npc: Mutex::new(None),
            pre_game: Mutex::new(None),

            rule_data: Mutex::new(None),
            card_data: Mutex::new(None),
            npc_data: Mutex::new(None),
            persist_data: Mutex::new(None),
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
