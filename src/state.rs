// use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::Path;

use chrono::naive::NaiveDateTime;
use chrono::prelude::Utc;

use crate::path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    // Today's kanji
    kanji: String,
    // Previously rolled kanji
    history: Vec<String>,
    // Last modified datestamp, should be compared to today
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

/// Load config from JSON file, initializing it if required
fn load_config() -> Result<Config, io::Error> {
    // Create default config if it does not exist
    // if !Path::new(CONFIG_PATH).exists() {
    if !Path::new(&path::get_config_path()).exists() {
        store(Config::default())?;
    }

    // let content = fs::read_to_string(CONFIG_PATH)?;
    let content = fs::read_to_string(&path::get_config_path())?;
    let parsed: Config = serde_json::from_str(&content)?;
    Ok(parsed)
}

/// Get previously rolled kanji from config
pub fn fetch_todays_kanji() -> String {
    load_config().unwrap().kanji
}

/// Set kanji symbol as the kanji for today
pub fn set_todays_kanji(kanji: &str) -> io::Result<()> {
    let mut cfg = load_config().unwrap();
    cfg.kanji = kanji.into();
    cfg.history.push(kanji.into());
    cfg.updated = Utc::now().naive_utc();
    store(cfg)
}

/// Check if last `updated` date is not today
pub fn should_roll_new_kanji() -> bool {
    let cfg = load_config().unwrap();
    let today = Utc::now().naive_utc();
    let diff = today.signed_duration_since(cfg.updated);
    diff.num_days() >= 1
}

/// Save config instance to file
fn store(config: Config) -> io::Result<()> {
    let json_data = serde_json::to_string(&config).unwrap();
    // fs::write(CONFIG_PATH, json_data)
    fs::write(&path::get_config_path(), json_data)
}
