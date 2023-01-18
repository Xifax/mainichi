use rand::seq::{IteratorRandom, SliceRandom};
use serde::Deserialize;
use slicer::AsSlicer;
use std::collections::HashMap;
use std::fs;
use std::io;
use thiserror::Error;

use colored::Colorize;

use crate::path;

/// Kanji as represented in JSON resource
#[derive(Deserialize, Clone, Debug, Default)]
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

/// Load {kanji: [chars]} DB
pub fn read_kanji_ordered_db() -> Result<String, Error> {
    let db_content = fs::read_to_string(path::get_kanji_ordedered_path())?;
    Ok(db_content)
}

/// Get random graded kanji
pub fn fetch_random_kanji_ranked() -> Kanji {
    let kanji_list = read_kanji_db().unwrap();
    kanji_list.choose(&mut rand::thread_rng()).unwrap().clone()
}

/// Get random graded kanji limited by frequency
pub fn fetch_random_kanji_ranked_by_frequency(max_frequency: usize) -> Kanji {
    let kanji_list = read_kanji_db().unwrap();
    (*kanji_list
        .iter()
        .filter(|k| k.frequency <= max_frequency)
        .collect::<Vec<&Kanji>>()
        .choose(&mut rand::thread_rng())
        .unwrap())
    .clone()
}

/// Get random kanji by simple positional index (higher -> rarer in Wiki)
pub fn fetch_random_kanji_ranked_by_position(max_frequency: usize) -> Kanji {
    let kanji_ordered = read_kanji_ordered_db().unwrap();
    let binding = kanji_ordered.chars().collect::<Vec<_>>();
    let kanji_char = binding
        .iter()
        .take(max_frequency)
        .choose(&mut rand::thread_rng());
    fetch_kanji(&kanji_char.unwrap().to_string())
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

/// Search everything
pub fn search_universal(query: &str, full_text: bool) -> Vec<String> {
    let kanji_list = read_kanji_db().unwrap();
    let found_gloss = kanji_list
        .iter()
        .filter(|k| k.gloss.contains(query))
        .map(|k| k.gloss.clone());

    let mut results: Vec<String> = vec![];

    for s in found_gloss.into_iter() {

        // Slice string until after the word
        let mut slicer = s.as_str().as_slicer();
        let before = slicer.slice_until(query).unwrap();
        slicer.skip_over(query);


        let after;
        let sentence;

        // Either get part of text (sentence|paragraph)
        if !full_text {
            after = slicer.slice_non_whitespace().unwrap();

            // Cut by newlines and get penultimate result -> our sentence with the word
            let mut pre = before.split_whitespace();

            // Try to get text preceding the query, until before the first newline
            sentence = if let Some(value) = pre.nth_back(1) {
                value
            } else {
                ""
            };

        // Or full entry
        } else {
            after = slicer.slice_to_end().unwrap();
            sentence = before;
        }

        // Combine it together and highlight found query
        let slice = format!("{}{}{}", sentence, query.red(), after);

        results.push(slice);
    }
    results
}
