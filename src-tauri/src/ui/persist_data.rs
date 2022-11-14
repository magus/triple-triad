use std::fs;
use std::path::Path;

use tauri::api::path;
use tauri::App;

// saved decks, serialized json
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PersistData {
    // from app state
    pub deck_list: Vec<Deck>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Deck {
    // from app state
    pub cards: Vec<String>,
}

impl PersistData {
    pub fn write(&self, app: &tauri::AppHandle) {
        let app_dir = path::app_dir(&app.config()).unwrap();
        let persist_json_path = Path::new(&app_dir).join("data").join("persist.json");

        let json = serde_json::to_string(&self).unwrap();
        println!("💾 saving persist data json");
        println!("[json={json}]");
        fs::write(persist_json_path, json).unwrap();
    }

    pub fn read(app: &App) -> PersistData {
        let app_dir = path::app_dir(&app.config()).unwrap();
        let data_dir = Path::new(&app_dir).join("data");
        let persist_json_path = Path::new(&data_dir).join("persist.json");

        println!("[app_dir={}]", app_dir.display());
        println!("[persist_json_path={}]", persist_json_path.display());

        if Path::exists(&persist_json_path) {
            println!("exists, read in json");
            let file = fs::File::open(persist_json_path).unwrap();

            let data: PersistData =
                serde_json::from_reader(file).expect("file should be proper JSON");

            // println!("[persist_data={:#?}]", data);

            return data;
        } else {
            println!("does not exist, write json");
            let deck_list = vec![];
            let data = PersistData { deck_list };
            let json = serde_json::to_string(&data).unwrap();

            // println!("[json={json}]");
            fs::create_dir_all(&data_dir).unwrap();
            fs::write(persist_json_path, json).unwrap();

            return data;
        }
    }
}
