use japanese::charset;
use lindera::Token;
use random_color::RandomColor;
use termion::color;
use wana_kana::to_hiragana::*;
use colourado::{ColorPalette, PaletteType};

use crate::json;
use crate::tokeniser;

/// Colorize part of sentence and its transcription with the same random color
fn colorize(part: &str, transcription: &str, highlight_kana: bool) -> (String, String) {
    // Don't format hiragana if required, and always don't format punctuation
    if !highlight_kana && charset::is_hiragana_string(part)
        || ["。", "、", "！", "『", "』", "…"].contains(&part)
    {
        let reset_color = color::Fg(color::Reset);
        let colored_part = format!("{reset_color}{part}");
        let colored_transcription = format!("{reset_color}{transcription}");

        (colored_part, colored_transcription)

    // Format everything else with matching colors
    } else {
        let random_color = RandomColor::new().to_rgb_array();
        let console_color = color::Fg(color::Rgb(
            random_color[0],
            random_color[1],
            random_color[2],
        ));

        let colored_part = format!("{console_color}{part}");
        let colored_transcription = format!("{console_color}{transcription}");

        (colored_part, colored_transcription)
    }
}

// TODO: colorize from palette???

/// Let's apply it for kanji transcription maybe? Both for words and transcriptions
/// Ideally, the random colors should match
/// So let's create [initial sentence] and [transcription sentence] side by side
/// e.g., new_random_color for token[0]~kanji and token[7]~reading
pub fn print_colorized(tokens: Vec<Token>, highlight_kana: bool) {
    let mut sentence = String::from("");
    let mut reading = String::from("");

    for token in tokens {
        let details = &token.details.unwrap();

        if details.len() > 7 {
            // Positions in token array correspond to different details
            // 6 => default form
            // 7 => reading
            let part = token.text;

            // Convert to hiragana only if not katakana
            let transcription = if part != details[7] {
                to_hiragana(&details[7].clone())
            } else {
                details[7].clone()
            };

            // Colorize each part differently
            let (colored_part, colored_transcription) =
                colorize(&part, &transcription, highlight_kana);
            sentence.push_str(&colored_part);
            reading.push_str(&colored_transcription);
        }
    }

    println!("{}", &sentence);
    println!("{}", &reading);
    println!("\n");
}


/// Transform vector of tokens to palleted colors rainbow madness
pub fn colorize_vec_to_str(tokenised_words: Vec<Token>) -> String {

    // number, type, closeness
    let mut palette = ColorPalette::new(tokenised_words.len() as u32, PaletteType::Pastel, true);
    let mut resulting_string = String::from("");

    // Iterate two arrays simultaneously by zipping them together:
    // (el0 from array0, el0 from array1)
    for it in tokenised_words.iter().zip(palette.colors.iter_mut()) {
        let (token, color) = it;

        // Skip token if punctuation, numeral and such (pastel looks nice even when its everywhere!)
        if token.text.chars().all(charset::is_japanese_punctuation) 
        || token.text.chars().all(charset::is_japanese_special_character) 
        || token.text.chars().all(char::is_numeric) 
        || charset::is_hiragana_string(&token.text) 
        {
            resulting_string.push_str(&format!("{}{}", color::Fg(color::Reset), token.text));
            continue;
        }

        // Increase brightness of all colors!
        let brightened_array: [f32; 3] = color.to_array().map(|x| x * 1.75_f32);
        // Convert 0.255 RGB values to 255 integer values
        let array: [u8; 3] = brightened_array.map(|x| (x * 100.0_f32) as u8);
        let console_color = color::Fg(color::Rgb(array[0], array[1], array[2]));

        let colored_word = format!("{}{}", console_color, token.text);
        resulting_string.push_str(&colored_word);
    }

    resulting_string
}

/// Print word and its info (customise)
pub fn print_word(word: &json::Word) {
    // Tokenise and display glossary entry
    tokenise_colorise(&word.gloss)
}

/// Morphologically split Japanese text and colorise its parts
pub fn tokenise_colorise(text: &str) {
    // Tokenise and display glossary entry
    let mut tokenizer = tokeniser::LinderaTokenizer::new();
    let tokens = tokenizer.tokenize(text);
    println!("{}", colorize_vec_to_str(tokens));
}
