use std::cmp::PartialEq;
use iced::Color;

/// The different color themes available.
pub enum ColorThemes {
    Light,
    Dark,
}
impl PartialEq for ColorThemes {
    /// Determines if two themes are equal.
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}
impl ColorThemes {
    /// Gets the name of the theme.
    pub fn name(&self) -> String {
        match self {
            ColorThemes::Light => "Light".to_string(),
            ColorThemes::Dark => "Dark".to_string(),
        }
    }
}



/// All the colors used in the application.
#[derive(Debug, Clone, Copy)]
pub enum Colors {
    // accent colors
    Accent,
    Amber,
    Apricot,
    Aqua,
    Blush,
    Butter,
    Honey,
    Lavender,
    Lilac,
    Mauve,
    Mint,
    Orchid,
    Peach,
    Periwinkle,
    Plum,
    Powder,
    Red,
    Rose,
    Sage,
    Salmon,
    Seafoam,
    Sky,
    Thistle,
    // function colors
    Text,
    Shadow,
    Foreground,
    Background,
}
impl PartialEq for Colors {
    /// Determines if two colors are equal.
    fn eq(&self, other: &Self) -> bool {
        let light_match = self.themed(ColorThemes::Light) == other.themed(ColorThemes::Light);
        let dark_match = self.themed(ColorThemes::Dark) == other.themed(ColorThemes::Dark);
        light_match && dark_match
    }
}
impl Colors {
    /// Gets a color from a hex value.
    pub fn from_hex(hex: u32) -> Color {
        Color::from_rgb(
            ((hex >> 16) & 0xFF) as f32 / 255.0,
            ((hex >> 8) & 0xFF) as f32 / 255.0,
            (hex & 0xFF) as f32 / 255.0,
        )
    }

    /// Gets the themed color for the color.
    pub fn themed(&self, theme: ColorThemes) -> Color {
        match self {
            Colors::Accent => {
                Colors::Aqua.themed(theme)
            }
            Colors::Amber => {
                match theme {
                    ColorThemes::Light => { Colors::from_hex(0xFFF4B8) }
                    ColorThemes::Dark => { Colors::from_hex(0xCCB85C) }
                }
            }
            Colors::Apricot => {
                match theme {
                    ColorThemes::Light => { Colors::from_hex(0xFFE6B8) }
                    ColorThemes::Dark => { Colors::from_hex(0xCC9B5C) }
                }
            }
            Colors::Aqua => {
                match theme {
                    ColorThemes::Light => { Colors::from_hex(0xB8E6F1) }
                    ColorThemes::Dark => { Colors::from_hex(0x5C9BB8) }
                }
            }
            Colors::Blush => {
                match theme {
                    ColorThemes::Light => { Colors::from_hex(0xF9D4E6) }
                    ColorThemes::Dark => { Colors::from_hex(0xB87B9B) }
                }
            }
            Colors::Butter => {
                match theme {
                    ColorThemes::Light => { Colors::from_hex(0xFFF1D4) }
                    ColorThemes::Dark => { Colors::from_hex(0xCCAD7B) }
                }
            }
            Colors::Honey => {
                match theme {
                    ColorThemes::Light => { Colors::from_hex(0xF9E6C5) }
                    ColorThemes::Dark => { Colors::from_hex(0xB89B6B) }
                }
            }
            Colors::Lavender => {
                match theme {
                    ColorThemes::Light => { Colors::from_hex(0xD4C5F9) }
                    ColorThemes::Dark => { Colors::from_hex(0x7B5FB8) }
                }
            }
            Colors::Lilac => {
                match theme {
                    ColorThemes::Light => { Colors::from_hex(0xE6D4F1) }
                    ColorThemes::Dark => { Colors::from_hex(0x9B7BB8) }
                }
            }
            Colors::Mauve => {
                match theme {
                    ColorThemes::Light => { Colors::from_hex(0xE6B8D4) }
                    ColorThemes::Dark => { Colors::from_hex(0x9B5F7B) }
                }
            }
            Colors::Mint => {
                match theme {
                    ColorThemes::Light => { Colors::from_hex(0xB8F1D4) }
                    ColorThemes::Dark => { Colors::from_hex(0x4AA77B) }
                }
            }
            Colors::Orchid => {
                match theme {
                    ColorThemes::Light => { Colors::from_hex(0xF1C5E6) }
                    ColorThemes::Dark => { Colors::from_hex(0xB86B9B) }
                }
            }
            Colors::Peach => {
                match theme {
                    ColorThemes::Light => { Colors::from_hex(0xFFD4B8) }
                    ColorThemes::Dark => { Colors::from_hex(0xCC8A5C) }
                }
            }
            Colors::Periwinkle => {
                match theme {
                    ColorThemes::Light => { Colors::from_hex(0xC5D4F9) }
                    ColorThemes::Dark => { Colors::from_hex(0x6B7BB8) }
                }
            }
            Colors::Plum => {
                match theme {
                    ColorThemes::Light => { Colors::from_hex(0xD9B8E6) }
                    ColorThemes::Dark => { Colors::from_hex(0x8A5F99) }
                }
            }
            Colors::Powder => {
                match theme {
                    ColorThemes::Light => { Colors::from_hex(0xD4E6F1) }
                    ColorThemes::Dark => { Colors::from_hex(0x7BA7CC) }
                }
            }
            Colors::Red => {
                match theme {
                    ColorThemes::Light => { Colors::from_hex(0xFFB8C5) }
                    ColorThemes::Dark => { Colors::from_hex(0xCC6B7C) }
                }
            }
            Colors::Rose => {
                match theme {
                    ColorThemes::Light => { Colors::from_hex(0xF1B8D4) }
                    ColorThemes::Dark => { Colors::from_hex(0xB85F8A) }
                }
            }
            Colors::Sage => {
                match theme {
                    ColorThemes::Light => { Colors::from_hex(0xC5E6D0) }
                    ColorThemes::Dark => { Colors::from_hex(0x6B9B7C) }
                }
            }
            Colors::Salmon => {
                match theme {
                    ColorThemes::Light => { Colors::from_hex(0xFFCAB8) }
                    ColorThemes::Dark => { Colors::from_hex(0xCC7F5C) }
                }
            }
            Colors::Seafoam => {
                match theme {
                    ColorThemes::Light => { Colors::from_hex(0xB8F1E6) }
                    ColorThemes::Dark => { Colors::from_hex(0x5CB8A3) }
                }
            }
            Colors::Sky => {
                match theme {
                    ColorThemes::Light => { Colors::from_hex(0xB8D4F1) }
                    ColorThemes::Dark => { Colors::from_hex(0x4A7BA7) }
                }
            }
            Colors::Thistle => {
                match theme {
                    ColorThemes::Light => { Colors::from_hex(0xE6C5F1) }
                    ColorThemes::Dark => { Colors::from_hex(0x9B6FB8) }
                }
            }
            Colors::Text => {
                if theme == ColorThemes::Light { Colors::Background.themed(ColorThemes::Dark) }
                else { Colors::Foreground.themed(ColorThemes::Light) }
            }
            Colors::Shadow => {
                match theme {
                    ColorThemes::Light => { Colors::from_hex(0x3F464B) }
                    ColorThemes::Dark => { Colors::from_hex(0x272B2E) }
                }
            }
            Colors::Foreground => {
                match theme {
                    ColorThemes::Light => { Colors::from_hex(0xFFEFD4) }
                    ColorThemes::Dark => { Colors::from_hex(0x343D47) }
                }
            }
            Colors::Background => {
                match theme {
                    ColorThemes::Light => { Colors::from_hex(0xFFF5E5) }
                    ColorThemes::Dark => { Colors::from_hex(0x283641) }
                }
            }
        }
    }
}