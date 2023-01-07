use clap::Parser;

#[allow(unused)]
use colored::Colorize;

mod ascii;
mod cli;
mod json;
mod massif;
mod path;
mod service;
mod state;
mod terminal;
mod tokeniser;

mod pending;

#[tokio::main]
async fn main() {
    // let config = state::config();
    let args = cli::Args::parse();

    match args.action {
        //////////////////////////////////////////
        // Get new kanji or show already rolled //
        //////////////////////////////////////////
        cli::Action::Roll {
            force,
            max_frequency,
            ascii_art,
        } => {
            // Check for `forced` flags and so on
            let kanji: json::Kanji;
            if force || state::should_roll_new_kanji() {

                kanji = if let Some(frequency) = max_frequency {
                    json::fetch_random_kanji_ranked_by_frequency(frequency)
                } else {
                    json::fetch_random_kanji_ranked()
                };

                // kanji = json::fetch_random_kanji_ranked();

                // TODO: check if this kanji is not in history
                // TODO: (optional) check max_frequency
                // TODO: save max frequency when it's specified!
                // TODO: (optional) check frequency diff with last rolled kanji
                state::set_todays_kanji(&kanji.kanji).unwrap();
            } else {
                let kanji_symbol = state::fetch_todays_kanji();
                kanji = json::fetch_kanji(&kanji_symbol);
            }

            // Format prettily
            if ascii_art {
                ascii::text_to_ascii(&kanji.kanji);
            } else {
                println!("{}", kanji.kanji);
            }
        }
        // Display glossary definitions
        cli::Action::Gloss {
            colorize_kana,
            all_color,
        } => {
            let kanji = state::fetch_todays_kanji();
            let kanji: json::Kanji = json::fetch_kanji(&kanji);
            terminal::tokenise_colorise(&kanji.gloss, colorize_kana, all_color)
        }
        cli::Action::Words {
            colorize_kana,
            all_color,
        } => {
            let kanji = state::fetch_todays_kanji();
            let words = json::fetch_related_words(&kanji);

            if words.is_empty() {
                println!("No words found, sorry >.<");
                return;
            }

            for word in words.iter() {
                terminal::print_word(word, colorize_kana, all_color);
            }
        }
        //////////////////////////////////////
        // Fetch examples for today's kanji //
        //////////////////////////////////////
        cli::Action::Examples {
            count,
            randomize,
            highlight_kana,
            query,
            words_related,
        } => {
            // Fetch examples for related words
            if words_related {
                let kanji = state::fetch_todays_kanji();
                let words = json::fetch_related_words(&kanji);
                for word in words.iter() {
                    service::lookup_and_print_examples(
                        &word.word,
                        count,
                        randomize,
                        highlight_kana,
                    ).await;
                }
                return;
            }

            // Or lookup specific query | current kanji
            let lookup = if let Some(value) = query {
                value
            } else {
                state::fetch_todays_kanji()
            };

            service::lookup_and_print_examples(&lookup, count, randomize, highlight_kana).await;
        }
        //////////////////////////////
        // Additional functionality //
        //////////////////////////////
        cli::Action::Related {} => {
            let kanji = state::fetch_todays_kanji();
            let related_kanji = json::fetch_related_kanji(&kanji);
            match related_kanji {
                Some(group) => println!("{:#?}", group.group),
                None => println!("Sorry, nothing found~"),
            }
        }
        cli::Action::History {} => {
            for kanji in state::fetch_history() {
                println!("{kanji}")
            }
        }
        cli::Action::Lookup {} => {
            todo!()
        }
        ////////////////////////////////////////
        // Quick test functionality goes here //
        ////////////////////////////////////////
        cli::Action::Test => {}
    }
}
