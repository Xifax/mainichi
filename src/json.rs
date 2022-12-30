use rand::seq::SliceRandom;
use std::fs;
use std::io;
use thiserror::Error;

const DB_PATH: &str = "./data/kanji.json";

#[derive(Error, Debug)]
pub enum Error {
    #[error("error reading the DB file: {0}")]
    ReadDBError(#[from] io::Error),
    #[error("error parsing the DB file: {0}")]
    ParseDBError(#[from] serde_json::Error),
}

pub fn read_db() -> Result<Vec<String>, Error> {
    let db_content = fs::read_to_string(DB_PATH)?;
    let parsed: Vec<String> = serde_json::from_str(&db_content)?;
    Ok(parsed)
}

pub fn fetch_random_kanji() -> String {
    let kanji = read_db().unwrap();
    kanji.choose(&mut rand::thread_rng()).unwrap().clone()
}
