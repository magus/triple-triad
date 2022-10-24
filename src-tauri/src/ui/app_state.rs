use std::sync::Mutex;

use crate::game::Game;

// use interior mutability since the instance of AppState cannot change
// that means we Mutex fields so we can change their values
// this gives us a way to edit shared game state
// accessible via commands (invoke) and also rust app handles
#[derive(Clone, serde::Serialize)]
pub struct AppStateJson {
    status: String,
    game: Game,
}

pub struct AppState {
    pub status: Mutex<String>,
    pub game: Mutex<Game>,
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

    pub fn new() -> AppState {
        AppState {
            status: Mutex::new("setup".into()),
            game: Mutex::new(Game::new()),
        }
    }
}
