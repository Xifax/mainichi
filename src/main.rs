use clap::{Parser, Subcommand};

use colored::Colorize;

mod json;
mod state;
mod massif;
mod terminal;
mod tokeniser;

mod pending;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand)]
enum Action {
    /// Display kanji for today
    Roll {
        // TODO: !!!
        // // WIP: possible options
        // max_rarity: usize,
        // // Don't roll new kanji that is much rarer that previous one (sic!)
        // sort_by_rarity: bool,
    },
    Gloss {
        // Definition and so on
    },
    Words {
        // TODO:
        // related words

    },
    /// Display example for today's kanji
    Examples {
        #[clap(short, long, default_value_t = 6)]
        /// Number of examples to fetch
        count: usize,
        #[clap(long, default_value_t = false)]
        /// Highlight hiragana
        highlight_kana: bool,
    },
    /// Test functionality (move to tests?)
    Test,
}

fn main() {
    // let config = state::config();
    let args = Args::parse();

    match args.action {
        // Quick test functionality goes here
        Action::Test => {
            // pending::test_functionality();
            // let kanji = state::get_kanji();
            // dbg!(kanji);
            // let kanji = json::fetch_random_kanji_ranked();
            // state::set_kanji(&kanji.kanji);
            dbg!(state::should_roll_new_kanji());
        }
        // TODO: display glossary definitions
        Action::Gloss {} => {}
        // Get new kanji or show already rolled
        Action::Roll {} => {
            // let kanji = json::read_db().unwrap();
            // println!("{:?}", kanji.choose(&mut rand::thread_rng()));
            // TODO: save kanji as TODAY's kanji
            let kanji = json::fetch_random_kanji_ranked();
            println!("{:#?}", kanji);
            // println!("{:#?}", kanji.info().kanji);

        }
        Action::Words {} => {
            let kanji = json::fetch_random_kanji_ranked();
            let words = json::fetch_related_words(&kanji.kanji);
            println!("{:#?}", words);
        }
        // fetch examples for today's kanji
        Action::Examples {
            count,
            highlight_kana,
        } => {
            // TODO: get rolled kanji or roll new one
            let kanji = json::fetch_random_kanji();
            println!("{}\n", &kanji.red());

            // Fetch from Massif's API
            // TODO: indeterminate progressbar
            let response = massif::fetch_examples(&kanji).unwrap();

            // TODO: fetch examples in random order (check max size)
            for example in response.results.iter().take(count) {
                let mut tokenizer = tokeniser::LinderaTokenizer::new();
                let tokens = tokenizer.tokenize(&example.text);

                // print the sentence + reading, prettily~
                terminal::print_colorized(tokens, highlight_kana);
            }
        }
    }
}
