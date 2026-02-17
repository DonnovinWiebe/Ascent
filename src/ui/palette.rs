use std::cmp::PartialEq;
use iced::{Color, Theme};
use iced::theme::Palette;
use crate::ui::palette::ColorModes::*;

/// The different color modes available (Light and Dark).
pub enum ColorModes {
    Light,
    Dark,
}
impl PartialEq for ColorModes {
    /// Determines if two color modes are equal.
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}
impl ColorModes {
    /// Gets the name of the color mode.
    pub fn name(&self) -> String {
        match self {
            Light => "Light".to_string(),
            Dark => "Dark".to_string(),
        }
    }

    /// Gets the opposite color mode.
    pub fn opposite(&self) -> ColorModes {
        match self {
            Light => Dark,
            Dark => Light,
        }
    }
}



/// All the colors used in the application.
#[derive(Debug, Clone, Copy)]
pub enum ThemeColors {
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
    Shadow,
    Sky,
    Thistle,
    // function colors
    Text,
    Foreground,
    Background,
}
impl PartialEq for ThemeColors {
    /// Determines if two colors are equal.
    fn eq(&self, other: &Self) -> bool {
        let light_match = self.at(&Light) == other.at(&Light);
        let dark_match = self.at(&Dark) == other.at(&Dark);
        light_match && dark_match
    }
}
impl ThemeColors {
    /// Gets a color from a hex value.
    pub fn color_from_hex(hex: u32) -> Color {
        Color::from_rgb(
            ((hex >> 16) & 0xFF) as f32 / 255.0,
            ((hex >> 8) & 0xFF) as f32 / 255.0,
            (hex & 0xFF) as f32 / 255.0,
        )
    }

    /// Gets the themed color for the color.
    pub fn at(&self, color_mode: &ColorModes) -> Color {
        match self {
            ThemeColors::Accent => {
                ThemeColors::Aqua.at(color_mode)
            }
            ThemeColors::Amber => {
                match color_mode {
                    Light => { ThemeColors::color_from_hex(0xFFF4B8) }
                    Dark => { ThemeColors::color_from_hex(0xCCB85C) }
                }
            }
            ThemeColors::Apricot => {
                match color_mode {
                    Light => { ThemeColors::color_from_hex(0xFFE6B8) }
                    Dark => { ThemeColors::color_from_hex(0xCC9B5C) }
                }
            }
            ThemeColors::Aqua => {
                match color_mode {
                    Light => { ThemeColors::color_from_hex(0xB8E6F1) }
                    Dark => { ThemeColors::color_from_hex(0x5C9BB8) }
                }
            }
            ThemeColors::Blush => {
                match color_mode {
                    Light => { ThemeColors::color_from_hex(0xF9D4E6) }
                    Dark => { ThemeColors::color_from_hex(0xB87B9B) }
                }
            }
            ThemeColors::Butter => {
                match color_mode {
                    Light => { ThemeColors::color_from_hex(0xFFF1D4) }
                    Dark => { ThemeColors::color_from_hex(0xCCAD7B) }
                }
            }
            ThemeColors::Honey => {
                match color_mode {
                    Light => { ThemeColors::color_from_hex(0xF9E6C5) }
                    Dark => { ThemeColors::color_from_hex(0xB89B6B) }
                }
            }
            ThemeColors::Lavender => {
                match color_mode {
                    Light => { ThemeColors::color_from_hex(0xD4C5F9) }
                    Dark => { ThemeColors::color_from_hex(0x7B5FB8) }
                }
            }
            ThemeColors::Lilac => {
                match color_mode {
                    Light => { ThemeColors::color_from_hex(0xE6D4F1) }
                    Dark => { ThemeColors::color_from_hex(0x9B7BB8) }
                }
            }
            ThemeColors::Mauve => {
                match color_mode {
                    Light => { ThemeColors::color_from_hex(0xE6B8D4) }
                    Dark => { ThemeColors::color_from_hex(0x9B5F7B) }
                }
            }
            ThemeColors::Mint => {
                match color_mode {
                    Light => { ThemeColors::color_from_hex(0xB8F1D4) }
                    Dark => { ThemeColors::color_from_hex(0x4AA77B) }
                }
            }
            ThemeColors::Orchid => {
                match color_mode {
                    Light => { ThemeColors::color_from_hex(0xF1C5E6) }
                    Dark => { ThemeColors::color_from_hex(0xB86B9B) }
                }
            }
            ThemeColors::Peach => {
                match color_mode {
                    Light => { ThemeColors::color_from_hex(0xFFD4B8) }
                    Dark => { ThemeColors::color_from_hex(0xCC8A5C) }
                }
            }
            ThemeColors::Periwinkle => {
                match color_mode {
                    Light => { ThemeColors::color_from_hex(0xC5D4F9) }
                    Dark => { ThemeColors::color_from_hex(0x6B7BB8) }
                }
            }
            ThemeColors::Plum => {
                match color_mode {
                    Light => { ThemeColors::color_from_hex(0xD9B8E6) }
                    Dark => { ThemeColors::color_from_hex(0x8A5F99) }
                }
            }
            ThemeColors::Powder => {
                match color_mode {
                    Light => { ThemeColors::color_from_hex(0xD4E6F1) }
                    Dark => { ThemeColors::color_from_hex(0x7BA7CC) }
                }
            }
            ThemeColors::Red => {
                match color_mode {
                    Light => { ThemeColors::color_from_hex(0xFFB8C5) }
                    Dark => { ThemeColors::color_from_hex(0xCC6B7C) }
                }
            }
            ThemeColors::Rose => {
                match color_mode {
                    Light => { ThemeColors::color_from_hex(0xF1B8D4) }
                    Dark => { ThemeColors::color_from_hex(0xB85F8A) }
                }
            }
            ThemeColors::Sage => {
                match color_mode {
                    Light => { ThemeColors::color_from_hex(0xC5E6D0) }
                    Dark => { ThemeColors::color_from_hex(0x6B9B7C) }
                }
            }
            ThemeColors::Salmon => {
                match color_mode {
                    Light => { ThemeColors::color_from_hex(0xFFCAB8) }
                    Dark => { ThemeColors::color_from_hex(0xCC7F5C) }
                }
            }
            ThemeColors::Seafoam => {
                match color_mode {
                    Light => { ThemeColors::color_from_hex(0xB8F1E6) }
                    Dark => { ThemeColors::color_from_hex(0x5CB8A3) }
                }
            }
            ThemeColors::Shadow => {
                match color_mode {
                    Light => { ThemeColors::color_from_hex(0x3F464B) }
                    Dark => { ThemeColors::color_from_hex(0x272B2E) }
                }
            }
            ThemeColors::Sky => {
                match color_mode {
                    Light => { ThemeColors::color_from_hex(0xB8D4F1) }
                    Dark => { ThemeColors::color_from_hex(0x4A7BA7) }
                }
            }
            ThemeColors::Thistle => {
                match color_mode {
                    Light => { ThemeColors::color_from_hex(0xE6C5F1) }
                    Dark => { ThemeColors::color_from_hex(0x9B6FB8) }
                }
            }
            ThemeColors::Text => {
                ThemeColors::Background.at(&color_mode.opposite())
            }
            ThemeColors::Foreground => {
                match color_mode {
                    Light => { ThemeColors::color_from_hex(0xFFEFD4) }
                    Dark => { ThemeColors::color_from_hex(0x343D47) }
                }
            }
            ThemeColors::Background => {
                match color_mode {
                    Light => { ThemeColors::color_from_hex(0xFFF5E5) }
                    Dark => { ThemeColors::color_from_hex(0x283641) }
                }
            }
        }
    }
}



/// The different themes available.
pub enum Themes {
    Peach,
    Midnight,
    Sunrise,
    Ocean,
}
impl Themes {
    /// Gets the theme's name.
    pub fn name(&self) -> String {
        match self {
            Themes::Peach => { "Peach".to_string() }
            Themes::Midnight => { "Midnight".to_string() }
            Themes::Sunrise => { "Sunrise".to_string() }
            Themes::Ocean => { "Ocean".to_string() }
        }
    }

    /// Creates a palette for an Iced Theme.
    pub fn generate(&self) -> Theme {
        let palette = Palette {
            background: self.background(),
            text: self.text(),
            primary: self.primary(),
            success: self.success(),
            warning: self.warning(),
            danger: self.danger(),
        };

        Theme::custom(self.name(), palette)
    }

    /// Gets the color mode used for the theme.
    pub fn color_mode(&self) -> ColorModes {
        match self {
            Themes::Peach => { Light }
            Themes::Midnight => { Dark }
            Themes::Sunrise => { Light }
            Themes::Ocean => { Dark }
        }
    }

    /// Gets the theme's background color.
    fn background(&self) -> Color {
        match self {
            Themes::Peach => { ThemeColors::Background.at(&self.color_mode()) }
            Themes::Midnight => { ThemeColors::Background.at(&self.color_mode()) }
            Themes::Sunrise => { ThemeColors::Salmon.at(&self.color_mode()) }
            Themes::Ocean => { ThemeColors::Periwinkle.at(&self.color_mode()) }
        }
    }

    /// Gets the theme's text color.
    fn text(&self) -> Color {
        match self {
            Themes::Peach => { ThemeColors::Text.at(&self.color_mode()) }
            Themes::Midnight => { ThemeColors::Text.at(&self.color_mode()) }
            Themes::Sunrise => { ThemeColors::Text.at(&self.color_mode()) }
            Themes::Ocean => { ThemeColors::Text.at(&self.color_mode()) }
        }
    }

    /// Gets the theme's primary color.
    fn primary(&self) -> Color {
        match self {
            Themes::Peach => { ThemeColors::Foreground.at(&self.color_mode()) }
            Themes::Midnight => { ThemeColors::Foreground.at(&self.color_mode()) }
            Themes::Sunrise => { ThemeColors::Butter.at(&self.color_mode()) }
            Themes::Ocean => { ThemeColors::Aqua.at(&self.color_mode()) }
        }
    }

    /// Gets the theme's success color.
    fn success(&self) -> Color {
        match self {
            Themes::Peach => { ThemeColors::Sage.at(&self.color_mode()) }
            Themes::Midnight => { ThemeColors::Sage.at(&self.color_mode()) }
            Themes::Sunrise => { ThemeColors::Apricot.at(&self.color_mode()) }
            Themes::Ocean => { ThemeColors::Seafoam.at(&self.color_mode()) }
        }
    }

    /// Gets the theme's warning color.
    fn warning(&self) -> Color {
        match self {
            Themes::Peach => { ThemeColors::Apricot.at(&self.color_mode()) }
            Themes::Midnight => { ThemeColors::Apricot.at(&self.color_mode()) }
            Themes::Sunrise => { ThemeColors::Peach.at(&self.color_mode()) }
            Themes::Ocean => { ThemeColors::Thistle.at(&self.color_mode()) }
        }
    }

    /// Gets the theme's danger color.
    fn danger(&self) -> Color {
        match self {
            Themes::Peach => { ThemeColors::Red.at(&self.color_mode()) }
            Themes::Midnight => { ThemeColors::Red.at(&self.color_mode()) }
            Themes::Sunrise => { ThemeColors::Red.at(&self.color_mode()) }
            Themes::Ocean => { ThemeColors::Mauve.at(&self.color_mode()) }
        }
    }
}