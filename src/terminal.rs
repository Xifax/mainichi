use random_color::RandomColor;
use termion::color;
use lindera::Token;
use wana_kana::to_hiragana::*;

fn colorize(part: &str, transcription: &str) -> (String, String){
    let random_color = RandomColor::new().to_rgb_array();
    let console_color = color::Fg(color::Rgb(random_color[0], random_color[1], random_color[2]));

    let colored_part = format!("{}{}", part, console_color);
    let colored_transcription = format!("{}{}", transcription, console_color);

    (colored_part, colored_transcription)
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
        // println!("{:#?}", details);
        if details.len() > 7 {
            // Positions in token array correspond to different details
            // 6 => default form
            // 7 => reading
            let part = token.text;
            // let transcription = &to_hiragana(&details[7]);

            // Convert to hiragana only if not katakana
            let transcription = if &part != &details[7] {
                to_hiragana(&details[7].clone())
            } else {
                details[7].clone()
            };

            // colorize each part differently
            let (colored_part, colored_transcription) = colorize(&part, &transcription);
            sentence.push_str(&colored_part);
            reading.push_str(&colored_transcription);
        }
    }

    println!("{}", &sentence);
    println!("{}", &reading);
    println!("\n");
}
