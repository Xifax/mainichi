use colourado::{ColorPalette, PaletteType};
use termion::color;

#[allow(unused)]
pub fn test_functionality() {
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
