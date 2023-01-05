use rand::seq::SliceRandom;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::io;
use thiserror::Error;

use crate::path;

/// Kanji as represented in JSON resource
#[derive(Deserialize, Clone, Debug)]
pub struct Kanji {
    pub kanji: String,
    pub reading: String,
    pub gloss: String,
    pub frequency: usize,
}

/// Word formed with multiple kanji as represented in JSON resource
#[derive(Deserialize, Clone, Debug)]
pub struct Word {
    pub word: String,
    pub reading: String,
    pub gloss: String,
    pub frequency: usize,
}

/// Kanji group as represented in JSON resource
#[derive(Deserialize, Clone, Debug)]
pub struct KanjiGroup {
    pub kanji: String,
    pub group: Vec<String>,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("error reading the DB file: {0}")]
    ReadDBError(#[from] io::Error),
    #[error("error parsing the DB file: {0}")]
    ParseDBError(#[from] serde_json::Error),
}

/// Load graded [kanji] DB
pub fn read_kanji_db() -> Result<Vec<Kanji>, Error> {
    let db_content = fs::read_to_string(path::get_kanji_path())?;
    let parsed: Vec<Kanji> = serde_json::from_str(&db_content)?;
    Ok(parsed)
}

/// Load {kanji: [words]} DB
pub fn read_words_db() -> Result<HashMap<String, Vec<Word>>, Error> {
    let db_content = fs::read_to_string(path::get_words_path())?;
    let parsed: HashMap<String, Vec<Word>> = serde_json::from_str(&db_content)?;
    Ok(parsed)
}

/// Load {kanji: [groups]} DB
pub fn read_groups_db() -> Result<HashMap<String, KanjiGroup>, Error> {
    let db_content = fs::read_to_string(path::get_kanji_groups_path())?;
    let parsed: HashMap<String, KanjiGroup> = serde_json::from_str(&db_content)?;
    Ok(parsed)
}

/// Get random graded kanji with frequency options
pub fn fetch_random_kanji_ranked() -> Kanji {
    // TODO: introduce max_frequency?
    let kanji_list = read_kanji_db().unwrap();
    kanji_list.choose(&mut rand::thread_rng()).unwrap().clone()
}

/// Get kanji by key
pub fn fetch_kanji(kanji: &str) -> Kanji {
    let kanji_list = read_kanji_db().unwrap();
    kanji_list
        .iter()
        .find(|k| k.kanji == kanji)
        .unwrap()
        .clone()
}

/// Get example words for specified kanji (if any)
pub fn fetch_related_words(kanji: &str) -> Vec<Word> {
    let words = read_words_db().unwrap();
    words.get(kanji).unwrap().clone()
}

/// Get related kanji
pub fn fetch_related_kanji(kanji: &str) -> Option<KanjiGroup> {
    let groups = read_groups_db().unwrap();
    groups.get(kanji).cloned()
}
