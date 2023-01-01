use lindera::Token;
use random_color::RandomColor;
use termion::color;
use wana_kana::to_hiragana::*;

fn colorize(part: &str, transcription: &str) -> (String, String) {

    // Don't format punctuation
    if ["。", "、", "！", "『", "』"].contains(&part) {
        let reset_color = color::Fg(color::Reset);
        let colored_part = format!("{}{}", reset_color, part);
        let colored_transcription = format!("{}{}", reset_color, transcription);

        (colored_part, colored_transcription)
    } else {

        let random_color = RandomColor::new().to_rgb_array();
        let console_color = color::Fg(color::Rgb(
            random_color[0],
            random_color[1],
            random_color[2],
        ));

        let colored_part = format!("{}{}", console_color, part);
        let colored_transcription = format!("{}{}", console_color, transcription);

        (colored_part, colored_transcription)
    }

}

/// Let's apply it for kanji transcription maybe? Both for words and transcriptions
/// Ideally, the random colors should match
/// So let's create [initial sentence] and [transcription sentence] side by side
/// e.g., new_random_color for token[0]~kanji and token[7]~reading
pub fn print_colorized(tokens: Vec<Token>) {
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
            let transcription = if &part != &details[7] {
                to_hiragana(&details[7].clone())
            } else {
                details[7].clone()
            };

            // colorize each part differently
            // FIX: the very first word is somehow always white?..
            // if ["。", "、", "！", "『", "』"].contains(&part.as_ref()) {
            //     let reset_color = color::Fg(color::Reset);
            //     sentence.push_str(&format!("{}", reset_color));
            //     reading.push_str(&format!("{}", reset_color));
            //     sentence.push_str(&part);
            //     reading.push_str(&transcription);
            // } else {
            //     let (colored_part, colored_transcription) = colorize(&part, &transcription);
            //     sentence.push_str(&colored_part);
            //     reading.push_str(&colored_transcription);
            // }
            let (colored_part, colored_transcription) = colorize(&part, &transcription);
            sentence.push_str(&colored_part);
            reading.push_str(&colored_transcription);

            // TODO: add color::Reset!!!
            // TODO: only for the last item 
            // let reset_color = color::Fg(color::Reset);
            // sentence.push_str(&format!("{}", reset_color));
            // reading.push_str(&format!("{}", reset_color));
        }
    }

    println!("{}", &sentence);
    println!("{}", &reading);
    println!("\n");
}
