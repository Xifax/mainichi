use clap::{Parser, Subcommand};
use rand::seq::SliceRandom;

#[allow(unused)]
use colored::Colorize;

mod json;
mod massif;
mod state;
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
        /// Force to fetch new kanji even if there's already one for today
        #[clap(short, long, default_value_t = false)]
        force: bool,
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
        /// Number of examples to fetch
        #[clap(short, long, default_value_t = 5)]
        count: usize,

        /// Highlight hiragana
        #[clap(long, default_value_t = false)]
        highlight_kana: bool,

        /// Get random COUNT examples from all those fetched from Massif
        #[clap(short, long, default_value_t = false)]
        randomize: bool,
    },
    /// Test functionality (move to tests?)
    Test,
    // TODO: examples for words!!!
    // TODO: history!!!
    // TODO: show gloss|exampes for specific kanji|words
}

fn main() {
    // let config = state::config();
    let args = Args::parse();

    match args.action {
        // Quick test functionality goes here
        Action::Test => {
            // pending::test_functionality();
            // dbg!(state::should_roll_new_kanji());
            pending::test_spinnders();
        }
        // TODO: display glossary definitions
        Action::Gloss {} => {
            let kanji = state::fetch_todays_kanji();
            let kanji: json::Kanji = json::fetch_kanji(&kanji);
            // TODO: print prettily
            // TODO: colorize?
            // println!("{}", kanji.kanji);
            println!("{}", kanji.gloss);
        }
        // Get new kanji or show already rolled
        Action::Roll { force } => {
            // let kanji = json::read_db().unwrap();
            // println!("{:?}", kanji.choose(&mut rand::thread_rng()));
            // TODO: save kanji as TODAY's kanji

            let kanji: json::Kanji;
            if force || state::should_roll_new_kanji() {
                kanji = json::fetch_random_kanji_ranked();
                state::set_todays_kanji(&kanji.kanji).unwrap();
            } else {
                let kanji_symbol = state::fetch_todays_kanji();
                kanji = json::fetch_kanji(&kanji_symbol);
            }
            // TODO: format prettily
            println!("{:#?}", kanji.kanji);
        }
        Action::Words {} => {
            let kanji = json::fetch_random_kanji_ranked();
            let words = json::fetch_related_words(&kanji.kanji);
            println!("{words:#?}");
        }
        // fetch examples for today's kanji
        Action::Examples {
            count,
            highlight_kana,
            randomize,
        } => {
            // TODO: check if should roll a new one?
            // let kanji = json::fetch_random_kanji();
            // kanji = json::fetch_kanji(&kanji_symbol);
            let kanji = state::fetch_todays_kanji();
            // println!("{}\n", &kanji.red());

            // Fetch from Massif's API
            let response = massif::fetch_examples(&kanji).unwrap();

            let examples: Vec<massif::Example>;
            // Fetch examples in random order (check max size)
            if randomize {
                examples = response.results.choose_multiple(&mut rand::thread_rng(), count).cloned().collect();
            } else {
                examples = response.results;
            }

            for example in examples.iter().take(count) {
                let mut tokenizer = tokeniser::LinderaTokenizer::new();
                let tokens = tokenizer.tokenize(&example.text);

                // Print the sentence + reading, prettily~
                terminal::print_colorized(tokens, highlight_kana);
            }
        }
    }
}
