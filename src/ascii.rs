use random_color::RandomColor;
use text_to_png::Color;
use text_to_png::TextRenderer;

use artem::convert;
use artem::options::OptionBuilder;
use std::{
    fs::{read, remove_file, File},
    io::{BufWriter, Write},
};

use crate::path;

const KANJI_FONT: usize = 52;
const TEMP_PNG: &str = "text.png";
// const CHOICE_COLOR: &str = "Dark Turuoise";

/// Create ASCII art based on a string (kanji character)
pub fn text_to_ascii(text: &str) {
    let font = read(path::get_font_path()).unwrap();
    let renderer = TextRenderer::try_new_with_ttf_font_data(font)
        .expect("Example font is definitely loadable");

    // Randomize color and render PNG of kanji text
    let random_color = RandomColor::new().to_rgb_array();
    let color = Color::new(random_color[0], random_color[1], random_color[2]);
    let result = renderer.render_text_to_png_data(text, KANJI_FONT, color);

    let png_data = result.unwrap();

    let output_file = File::create(TEMP_PNG).unwrap();

    {
        let mut writer = BufWriter::new(output_file);
        writer.write_all(&png_data.data).unwrap();
    }

    let image = image::open(TEMP_PNG).unwrap();

    let options = OptionBuilder::new().build();

    // if kanji_of_kanji {
    //     options.characters = format!(r#"{}あいうえおかきくけこさ
    //     しすせそたちつてと゛゜々"#, text).to_string();
    // }

    let ascii = convert(image, options);

    remove_file(TEMP_PNG).unwrap();

    println!("{ascii}");
}
