use clap::{Parser, Subcommand};
use rand::seq::SliceRandom;

use colored::Colorize;

// use lindera::tokenizer::Tokenizer;
// use lindera::{
//     mode::Mode,
//     tokenizer::{DictionaryConfig, TokenizerConfig},
// };

mod json;
mod massif;
mod terminal;
mod tokeniser;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand)]
enum Action {
    /// Display kanji for today
    Roll,
    /// Display example for today's kanji
    Examples,
    /// Test functionality (move to tests!)
    Test,
}

fn main() {
    let args = Args::parse();

    match args.action {
        Action::Test => {
        }
        Action::Roll => {
            let kanji = json::read_db().unwrap();
            println!("{:?}", kanji.choose(&mut rand::thread_rng()));
            // TODO: save kanji as TODAY's kanji
        }
        // TODO: fetch examples for today's kanji
        Action::Examples => {
            let kanji = json::fetch_random_kanji();
            println!("{}\n", &kanji.red());

            let response = massif::fetch_examples(&kanji).unwrap();
            // TODO: move '4' to arguments
            for example in response.results.iter().take(4) {

                let mut tokenizer = tokeniser::LinderaTokenizer::new();
                let tokens = tokenizer.tokenize(&example.text);

                // output the tokens
                terminal::print_colorized(tokens);

            }
        }
    }
}
