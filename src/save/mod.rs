

use serde::{Serialize, Deserialize};
use std::{fs::File, io::{BufReader, BufWriter}};


#[derive(Clone,Debug, Deserialize, Serialize)]
pub struct AppState {
    pub book_name: String,
    pub page: i32
}

impl Default for AppState {
    fn default() -> Self {
        Self { book_name: Default::default(), page: 1 }
    }
}


pub fn load_settings() -> Option<AppState> {
    let f = File::open("./settings.json");
    match f {
        Ok(f) => {
            let reader = BufReader::new(f);
            let app_state: AppState = serde_json::from_reader(reader).ok()?;
            Some(app_state)
        }
        Err(_) => {
            None
        }
    }
}

pub fn save_settings(app_state: AppState) {
    let f = File::options().create(true).write(true).truncate(true).open("./settings.json").unwrap();
    let writer = BufWriter::new(f);
    serde_json::to_writer(writer, &app_state).unwrap();

}
