// use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use std::io::Error;
use std::fs;
use std::path::Path;

use chrono::naive::NaiveDateTime;
use chrono::prelude::Utc;
// use std::io;


#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    kanji: String,
    history: Vec<String>,
    // TODO: last modified datestamp, should be compared to today
    updated: NaiveDateTime,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            kanji: "".into(),
            history: vec![],
            updated: Utc::now().naive_utc(),
        }
    }
}

// FIX: relative paths!
const CONFIG_PATH: &str = "./data/config.json";


fn load_config() -> Result<Config, Error> {
    // Create default config if it does not exist
    if !Path::new(CONFIG_PATH).exists() {
        store(Config::default());
    }
    
    let content = fs::read_to_string(CONFIG_PATH)?;
    let parsed: Config = serde_json::from_str(&content)?;
    Ok(parsed)
}

pub fn get_kanji() -> String {
    load_config().unwrap().kanji
}

pub fn set_kanji(kanji: &str) {
    let mut cfg = load_config().unwrap();
    cfg.kanji = kanji.into();
    cfg.history.push(kanji.into());
    cfg.updated = Utc::now().naive_utc();
    store(cfg);
}

/// Check if last `updated` date is not today
pub fn should_roll_new_kanji() -> bool {
    let cfg = load_config().unwrap();
    let today = Utc::now().naive_utc();
    let diff = today.signed_duration_since(cfg.updated);
    diff.num_days() >= 1
}

fn store(config: Config) {
    let json_data = serde_json::to_string(&config).unwrap();
    // TODO: fix Result
    fs::write(CONFIG_PATH, json_data);
}

