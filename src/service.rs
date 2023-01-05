/// Put common operations here
use crate::massif;
use crate::terminal;
use crate::tokeniser;
use rand::seq::SliceRandom;

pub fn lookup_and_print_examples(query: &str, count: usize, randomize: bool, highlight_kana: bool) {
    // Fetch from Massif's API
    let response = massif::fetch_examples(&query).unwrap();

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
