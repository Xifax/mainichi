use clap::{Parser, Subcommand};
use rand::seq::SliceRandom;

use colored::Colorize;

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
    /// Test functionality (move to tests?)
    Test,
}

fn main() {
    let args = Args::parse();

    match args.action {
        // Quick test functionality goes here
        Action::Test => {
        }
        // Get new kanji or show already rolled 
        Action::Roll => {
            let kanji = json::read_db().unwrap();
            println!("{:?}", kanji.choose(&mut rand::thread_rng()));
            // TODO: save kanji as TODAY's kanji
        }
        // fetch examples for today's kanji
        Action::Examples => {
            // TODO: get rolled kanji or roll new one
            let kanji = json::fetch_random_kanji();
            println!("{}\n", &kanji.red());

            // Fetch from Massif's API 
            // TODO: indeterminate progressbar
            let response = massif::fetch_examples(&kanji).unwrap();

            // TODO: move '4' to arguments
            for example in response.results.iter().take(4) {

                let mut tokenizer = tokeniser::LinderaTokenizer::new();
                let tokens = tokenizer.tokenize(&example.text);

                // print the sentence + reading prettely~
                terminal::print_colorized(tokens);
            }
        }
    }
}
