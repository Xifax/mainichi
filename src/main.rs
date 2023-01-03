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
    // Definition and so on
    Gloss {
        /// Display kana as colored, leave kanji white
        #[clap(short, long, default_value_t = false)]
        colorize_kana: bool,

        /// Colorize everything
        #[clap(long, default_value_t = false)]
        colorize_all: bool,
    },
    // Related words
    Words {
        /// Display kana as colored, leave kanji white
        #[clap(short, long, default_value_t = false)]
        colorize_kana: bool,

        /// Colorize everything
        #[clap(long, default_value_t = false)]
        colorize_all: bool,
    },
    /// Display example for today's kanji
    Examples {
        /// Number of examples to fetch
        #[clap(short, long, default_value_t = 5)]
        count: usize,

        /// Highlight ALL hiragana
        #[clap(long, default_value_t = false)]
        highlight_kana: bool,

        /// Get random COUNT examples from all those fetched from Massif
        #[clap(short, long, default_value_t = false)]
        randomize: bool,
    },
    History {
        // Show kanji history
    },
    Related {
        // Similar kanji and words
    },
    Lookup {
        // Lookup kanji|words|examples for provided query in local resources
        // TODO: examples for words go here?
        // TODO: show gloss|exampes for specific kanji|words
    },
    /// Test functionality
    Test,
}

fn main() {
    // let config = state::config();
    let args = Args::parse();

    match args.action {
        //////////////////////////////////////////
        // Get new kanji or show already rolled //
        //////////////////////////////////////////
        Action::Roll { force } => {
            // Check for `forced` flags and so on
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
        // Display glossary definitions
        Action::Gloss {
            colorize_kana,
            colorize_all,
        } => {
            let kanji = state::fetch_todays_kanji();
            let kanji: json::Kanji = json::fetch_kanji(&kanji);
            terminal::tokenise_colorise(&kanji.gloss, colorize_kana, colorize_all)
        }
        Action::Words {
            colorize_kana,
            colorize_all,
        } => {
            // let kanji = json::fetch_random_kanji_ranked();
            let kanji = state::fetch_todays_kanji();
            let words = json::fetch_related_words(&kanji);

            if words.is_empty() {
                println!("No words found, sorry >.<");
                return;
            }

            for word in words.iter() {
                terminal::print_word(word, colorize_kana, colorize_all);
            }
        }
        //////////////////////////////////////
        // Fetch examples for today's kanji //
        //////////////////////////////////////
        Action::Examples {
            count,
            highlight_kana,
            randomize,
        } => {
            let kanji = state::fetch_todays_kanji();

            // Fetch from Massif's API
            let response = massif::fetch_examples(&kanji).unwrap();

            // let examples: Vec<massif::Example>;
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

            for example in examples.iter().take(count) {
                let mut tokenizer = tokeniser::LinderaTokenizer::new();
                let tokens = tokenizer.tokenize(&example.text);

                // Print the sentence + reading, prettily~
                terminal::print_colorized(tokens, highlight_kana);
            }
        }
        //////////////////////////////
        // Additional functionality //
        //////////////////////////////
        Action::History {} => {
            todo!()
        }
        Action::Lookup {} => {
            todo!()
        }
        Action::Related {} => {
            todo!()
        }
        ////////////////////////////////////////
        // Quick test functionality goes here //
        ////////////////////////////////////////
        Action::Test => {
            // pending::test_functionality();
            // dbg!(state::should_roll_new_kanji());
            // pending::test_spinnders();
        }
    }
}
