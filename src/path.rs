use home::home_dir;
use relative_path::RelativePath;
use std::{env::current_dir, path::PathBuf};

/// Root path for sources and config
const DATA_PATH: &str = ".local/share/mainichi/";

/// Resources and cofig paths
const CONFIG_PATH: &str = "data/config.json";
pub const KANJI_PATH: &str = "data/kanji_ranked.json";
const KANJI_GROUP_PATH: &str = "data/kanji_group.json";
const WORDS_PATH: &str = "data/related_words_by_kanji.json";


pub fn get_relative_path(path: &str) -> String {
    let home: PathBuf = home_dir().unwrap();
    let relative_path = RelativePath::new(DATA_PATH);
    let path = relative_path.join(path).to_path(&home);
    path.into_os_string().into_string().unwrap().clone()
}

pub fn get_config_path() -> String {
    get_relative_path(CONFIG_PATH)
}

pub fn get_kanji_path() -> String {
    get_relative_path(KANJI_PATH)
}

pub fn get_kanji_group_path() -> String {
    get_relative_path(KANJI_GROUP_PATH)
}

pub fn get_words_path() -> String {
    get_relative_path(WORDS_PATH)
}
