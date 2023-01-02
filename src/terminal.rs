use japanese::charset;
use lindera::Token;
use random_color::RandomColor;
use termion::color;
use wana_kana::to_hiragana::*;

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
