use clap::{Parser, Subcommand};
use rand::seq::SliceRandom;

use colored::Colorize;
use colourado::{ColorPalette, PaletteType};
use termion::color;

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
    let args = Args::parse();

    match args.action {
        // Quick test functionality goes here
        Action::Test => {
            // number, type, closeness
            let palette = ColorPalette::new(25, PaletteType::Random, true);
            // let palette = ColorPalette::new(25, PaletteType::Pastel, false);
            for color in palette.colors {
                let color_array: [f32; 3] = color.to_array();
                let array: [u8; 3] = color_array.map(|x| (x * 100.0_f32) as u8);
                let console_color = color::Fg(color::Rgb(array[0], array[1], array[2]));

                let part = "俺はアイテムバッグ~ testo";
                let colored_part = format!("{console_color}{part}");
                println!("{colored_part}");
            }
        }
        // Get new kanji or show already rolled
        Action::Roll => {
            let kanji = json::read_db().unwrap();
            println!("{:?}", kanji.choose(&mut rand::thread_rng()));
            // TODO: save kanji as TODAY's kanji
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
