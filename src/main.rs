use clap::{Parser, Subcommand};
use rand::seq::SliceRandom;

use colored::Colorize;

use lindera::tokenizer::Tokenizer;
use lindera::LinderaResult;

mod json;
mod massif;

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
}

fn main() {
    let args = Args::parse();

    match args.action {
        Action::Roll => {
            let kanji = json::read_db().unwrap();
            println!("{:?}", kanji.choose(&mut rand::thread_rng()));
            // TODO: save kanji as TODAY's kanji
        }
        // TODO: fetch examples for today's kanji
        Action::Examples => {
            let kanji = json::fetch_random_kanji();
            println!("{}", &kanji.red());
            let response = massif::fetch_examples(&kanji).unwrap();
            for example in response.results.iter().take(3) {
                // let colored_example = example.text.replace(&kanji, &kanji.blue());
                // println!("{:#?}", example.text);
                // println!("{}", colored_example);

                // create tokenizer
                let tokenizer = Tokenizer::new()?;

                // tokenize the text
                let tokens = tokenizer.tokenize(example.text)?;

                // output the tokens
                for token in tokens {
                    println!("{}", token.text);
                }

            }

        }
    }
}
