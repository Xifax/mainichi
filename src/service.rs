/// Put common operations here
use crate::massif;
use crate::terminal;
use crate::tokeniser;
use rand::seq::SliceRandom;
use spinners::{Spinner, Spinners};

use crate::json;
use crate::state;

pub async fn lookup_and_print_examples(
    query: &str,
    count: usize,
    randomize: bool,
    highlight_kana: bool,
) {
    // Indeterminate progressbar
    let mut sp = Spinner::new(Spinners::Shark, format!("Fetching {query} from Massif..."));

    // Fetch from Massif's API
    let response = massif::fetch_examples(query).await.unwrap();

    // Stop progressbar and print newline
    sp.stop();
    println!("\n");

    // Fetch examples in random order (check max size)
    let examples = if randomize {
        response
            .results
            .choose_multiple(&mut rand::thread_rng(), count)
            .cloned()
            .collect()
    } else {
        response.results
    };

    if examples.is_empty() {
        println!("No examples found, sorry >.<");
        return;
    }

    for example in examples.iter().take(count) {
        let mut tokenizer = tokeniser::LinderaTokenizer::new();
        let tokens = tokenizer.tokenize(&example.text);

        // Print the sentence + reading, prettily~
        terminal::print_colorized(tokens, highlight_kana);
    }
}

/// Fetch new kanji according to specified options
/// Check if this kanji is not in history
/// Try N times until new kanji is found
/// Otherwise, return last found one
pub fn try_fetch_new_kanji(
    order_simple: bool,
    max_frequency: Option<usize>,
    max_tries: usize,
) -> json::Kanji {
    let mut kanji = json::Kanji::default();
    let mut current_try = 0;

    while state::already_in_history(&kanji.kanji) && current_try < max_tries {
        kanji = if let Some(frequency) = max_frequency {
            // Limit by position (first N kanji by frequency)
            if order_simple {
                json::fetch_random_kanji_ranked_by_position(frequency)
            // Limit by field (filter, frequency property <= N)
            } else {
                json::fetch_random_kanji_ranked_by_frequency(frequency)
            }
        } else {
            json::fetch_random_kanji_ranked()
        };

        current_try += 1;
    }

    kanji
}
