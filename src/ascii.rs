use text_to_png::TextRenderer;

use artem::convert;
use artem::options::OptionBuilder;
use std::{
    fs::{read, remove_file, File},
    io::{BufWriter, Write},
};

use crate::path;

pub fn text_to_ascii(text: &str) {
    let font = read(path::get_font_path()).unwrap();
    let renderer = TextRenderer::try_new_with_ttf_font_data(font)
        .expect("Example font is definitely loadable");

    // TODO: randomize color
    let result = renderer.render_text_to_png_data(text, 36, "Dark Turquoise");

    let png_data = result.unwrap();
    let output_path = "text.png";

    let output_file = File::create(output_path).unwrap();

    {
        let mut writer = BufWriter::new(output_file);
        writer.write_all(&png_data.data).unwrap();
    }

    let image = image::open(output_path).unwrap();
    let ascii = convert(image, OptionBuilder::new().build());

    remove_file(output_path).unwrap();

    println!("{ascii}");
}
