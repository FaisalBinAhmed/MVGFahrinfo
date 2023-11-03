use phf::phf_map;
use ratatui::style::Color;

static UBAHN_COLOR: phf::Map<&'static str, Color> = phf_map! {
    "U1" => Color::Rgb(60, 114, 53),
    "U2" => Color::Rgb(167, 45, 66),
    "U3" => Color::Rgb(235, 103, 32),
    "U4" => Color::Rgb(1, 171, 133),
    "U5" => Color::Rgb(190, 123, 0),
    "U6" => Color::Rgb(1, 101, 175),
    "U7" => Color::Rgb(79, 131, 43),
    "U8" => Color::Rgb(167, 45, 67),
};

static SBAHN_COLOR: phf::Map<&'static str, Color> = phf_map! {
    "S1" => Color::Rgb(16, 194, 233),
    "S2" => Color::Rgb(113, 193, 70),
    "S3" => Color::Rgb(118, 25, 113),
    "S4" => Color::Rgb(118, 25, 113),
    // "S5" => Color::Rgb(0, 0, 0),
    "S6" => Color::Rgb(3, 139, 79),
    "S7" => Color::Rgb(151, 53, 48),
    "S8" => Color::Rgb(0, 0, 0),
};

pub fn get_ubahn_color(keyword: &str) -> Color {
    return match UBAHN_COLOR.get(keyword) {
        Some(color) => *color,
        None => Color::Rgb(29, 43, 83),
    };
}

pub fn get_sbahn_color(keyword: &str) -> Color {
    return match SBAHN_COLOR.get(keyword) {
        Some(color) => *color,
        None => Color::Rgb(84, 253, 84),
    };
}
