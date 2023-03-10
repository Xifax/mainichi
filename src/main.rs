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
    let args = cli::Args::parse();

    match args.action {
        //////////////////////////////////////////
        // Get new kanji or show already rolled //
        //////////////////////////////////////////
        cli::Action::Roll {
            force,
            max_frequency,
            order_simple,
            ascii_art,
            set_kanji,
        } => {
            // Check for `forced` flags and so on
            let kanji: json::Kanji;
            // Kanji specified
            if set_kanji.is_some() {
                let kanji_symbol = set_kanji.unwrap();
                kanji = json::fetch_kanji(&kanji_symbol);
                state::set_todays_kanji(&kanji_symbol).unwrap();
            }
            // Get new kanji
            else if force || state::should_roll_new_kanji() {
                kanji = service::try_fetch_new_kanji(order_simple, max_frequency, 5);

                /*
                [Pending: advanced features]
                TODO: check if this kanji is not in history
                TODO: save max frequency when it's specified!
                TODO: (optional) check frequency diff with last rolled kanji
                */

                state::set_todays_kanji(&kanji.kanji).unwrap();
            // Get already saved kanji
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
            skip_gloss,
        } => {
            let kanji = state::fetch_todays_kanji();
            let words = json::fetch_related_words(&kanji);

            if words.is_empty() {
                println!("No words found, sorry >.<");
                return;
            }

            for word in words.iter() {
                terminal::print_word(word, colorize_kana, all_color, skip_gloss);
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
                let mut iter = words.iter().take(count).peekable();
                while let Some(word) = iter.next() {
                    service::lookup_and_print_examples(
                        &word.word,
                        count,
                        randomize,
                        highlight_kana,
                    )
                    .await;

                    if iter.peek().is_some() {
                        terminal::pause();
                    }
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
        cli::Action::Lookup { query, full } => {
            let results = json::search_universal(&query, full);
            for (r, is_last_element) in results
                .iter()
                .enumerate()
                // w will be true if it is last
                .map(|(i, w)| (w, i == results.len() - 1))
            {
                println!("{r}");
                // Separate by gray
                if !is_last_element {
                    println!("{}", format!("{}-", "-*".repeat(30)).truecolor(90, 90, 90));
                }
            }
        }
        cli::Action::Play { max_frequency } => {

            let frequency = if let Some(value) = max_frequency {
                value
            } else {
                2000
            };
            
            loop {
                let kanji = json::fetch_random_kanji_ranked_by_position(frequency);
                // 1. Display kanji ASCII
                ascii::text_to_ascii(&kanji.kanji);
                terminal::pause();
                // 2. Display kanji itself
                terminal::print_in_random_color(&kanji.kanji);
                terminal::pause();
                // 3. Show words
                let words = json::fetch_related_words(&kanji.kanji);
                // TODO: take N random
                for word in words.iter().take(4) {
                    // Highlight all, words only
                    terminal::print_word(word, false, true, true);
                }
                terminal::pause();
                // 4. Show info for words
                for word in words.iter().take(4) {
                    // Highlight all, words + gloss only
                    terminal::print_word(word, false, true, false);
                    terminal::pause();
                }
                // 5.1 Voice the examples using Naver API
                // todo!();
                // 5.2 Show examples
                service::lookup_and_print_examples(
                    &kanji.kanji,
                    2,
                    true,
                    false,
                )
                .await;
                terminal::pause();
                // 5.3 Show kanji gloss
                terminal::tokenise_colorise(&kanji.gloss, true, true);
                terminal::pause();
                // 6. IDEA: get kanji|words from the example
                // 6.X go to 1
            }
        }
        ////////////////////////////////////////
        // Quick test functionality goes here //
        ////////////////////////////////////////
        cli::Action::Test {} => {}
    }
}
