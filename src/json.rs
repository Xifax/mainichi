// use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use thiserror::Error;

// TODO: find kanji db (yomichan?) with a lot of stats and so on
// TODO: struct for relevant json
// #[derive(Serialize, Deserialize, Debug)]
// struct Kanji {
//     kanji: String,
// }

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
