use std::cmp::PartialEq;
use iced::{Color, Theme};
use iced::theme::Palette;
use crate::ui::palette::Appearance::*;

/// The different appearances available (Light and Dark).
pub enum Appearance {
    Light,
    Dark,
}
impl PartialEq for Appearance {
    /// Determines if two appearances are equal.
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}
impl Appearance {
    /// Gets the name of the appearances.
    pub fn name(&self) -> String {
        match self {
            Light => "Light".to_string(),
            Dark => "Dark".to_string(),
        }
    }

    /// Gets the opposite appearances.
    pub fn opposite(&self) -> Appearance {
        match self {
            Light => Dark,
            Dark => Light,
        }
    }
}



/// All the colors used in the application.
#[derive(Debug, Clone, Copy)]
pub enum AppColors {
    // accent colors
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
    Background,
    Text,
}
impl PartialEq for AppColors {
    /// Determines if two app colors are equal.
    fn eq(&self, other: &Self) -> bool {
        let light_match = self.at(Light) == other.at(Light);
        let dark_match = self.at(Dark) == other.at(Dark);
        light_match && dark_match
    }
}
impl AppColors {
    /// Gets an app color from a hex value.
    pub fn color_from_hex(hex: u32) -> Color {
        Color::from_rgb(
            ((hex >> 16) & 0xFF) as f32 / 255.0,
            ((hex >> 8) & 0xFF) as f32 / 255.0,
            (hex & 0xFF) as f32 / 255.0,
        )
    }

    /// Gets the app color based on an appearance.
    pub fn at(&self, color_mode: Appearance) -> Color {
        match self {
            AppColors::Amber => {
                match color_mode {
                    Light => { AppColors::color_from_hex(0xFFF4B8) }
                    Dark => { AppColors::color_from_hex(0xCCB85C) }
                }
            }
            AppColors::Apricot => {
                match color_mode {
                    Light => { AppColors::color_from_hex(0xFFE6B8) }
                    Dark => { AppColors::color_from_hex(0xCC9B5C) }
                }
            }
            AppColors::Aqua => {
                match color_mode {
                    Light => { AppColors::color_from_hex(0xB8E6F1) }
                    Dark => { AppColors::color_from_hex(0x5C9BB8) }
                }
            }
            AppColors::Blush => {
                match color_mode {
                    Light => { AppColors::color_from_hex(0xF9D4E6) }
                    Dark => { AppColors::color_from_hex(0xB87B9B) }
                }
            }
            AppColors::Butter => {
                match color_mode {
                    Light => { AppColors::color_from_hex(0xFFF1D4) }
                    Dark => { AppColors::color_from_hex(0xCCAD7B) }
                }
            }
            AppColors::Honey => {
                match color_mode {
                    Light => { AppColors::color_from_hex(0xF9E6C5) }
                    Dark => { AppColors::color_from_hex(0xB89B6B) }
                }
            }
            AppColors::Lavender => {
                match color_mode {
                    Light => { AppColors::color_from_hex(0xD4C5F9) }
                    Dark => { AppColors::color_from_hex(0x7B5FB8) }
                }
            }
            AppColors::Lilac => {
                match color_mode {
                    Light => { AppColors::color_from_hex(0xE6D4F1) }
                    Dark => { AppColors::color_from_hex(0x9B7BB8) }
                }
            }
            AppColors::Mauve => {
                match color_mode {
                    Light => { AppColors::color_from_hex(0xE6B8D4) }
                    Dark => { AppColors::color_from_hex(0x9B5F7B) }
                }
            }
            AppColors::Mint => {
                match color_mode {
                    Light => { AppColors::color_from_hex(0xB8F1D4) }
                    Dark => { AppColors::color_from_hex(0x4AA77B) }
                }
            }
            AppColors::Orchid => {
                match color_mode {
                    Light => { AppColors::color_from_hex(0xF1C5E6) }
                    Dark => { AppColors::color_from_hex(0xB86B9B) }
                }
            }
            AppColors::Peach => {
                match color_mode {
                    Light => { AppColors::color_from_hex(0xFFD4B8) }
                    Dark => { AppColors::color_from_hex(0xCC8A5C) }
                }
            }
            AppColors::Periwinkle => {
                match color_mode {
                    Light => { AppColors::color_from_hex(0xC5D4F9) }
                    Dark => { AppColors::color_from_hex(0x6B7BB8) }
                }
            }
            AppColors::Plum => {
                match color_mode {
                    Light => { AppColors::color_from_hex(0xD9B8E6) }
                    Dark => { AppColors::color_from_hex(0x8A5F99) }
                }
            }
            AppColors::Powder => {
                match color_mode {
                    Light => { AppColors::color_from_hex(0xD4E6F1) }
                    Dark => { AppColors::color_from_hex(0x7BA7CC) }
                }
            }
            AppColors::Red => {
                match color_mode {
                    Light => { AppColors::color_from_hex(0xFFB8C5) }
                    Dark => { AppColors::color_from_hex(0xCC6B7C) }
                }
            }
            AppColors::Rose => {
                match color_mode {
                    Light => { AppColors::color_from_hex(0xF1B8D4) }
                    Dark => { AppColors::color_from_hex(0xB85F8A) }
                }
            }
            AppColors::Sage => {
                match color_mode {
                    Light => { AppColors::color_from_hex(0xC5E6D0) }
                    Dark => { AppColors::color_from_hex(0x6B9B7C) }
                }
            }
            AppColors::Salmon => {
                match color_mode {
                    Light => { AppColors::color_from_hex(0xFFCAB8) }
                    Dark => { AppColors::color_from_hex(0xCC7F5C) }
                }
            }
            AppColors::Seafoam => {
                match color_mode {
                    Light => { AppColors::color_from_hex(0xB8F1E6) }
                    Dark => { AppColors::color_from_hex(0x5CB8A3) }
                }
            }
            AppColors::Shadow => {
                match color_mode {
                    Light => { AppColors::color_from_hex(0x3F464B) }
                    Dark => { AppColors::color_from_hex(0x272B2E) }
                }
            }
            AppColors::Sky => {
                match color_mode {
                    Light => { AppColors::color_from_hex(0xB8D4F1) }
                    Dark => { AppColors::color_from_hex(0x4A7BA7) }
                }
            }
            AppColors::Thistle => {
                match color_mode {
                    Light => { AppColors::color_from_hex(0xE6C5F1) }
                    Dark => { AppColors::color_from_hex(0x9B6FB8) }
                }
            }
            AppColors::Background => {
                match color_mode {
                    Light => { AppColors::color_from_hex(0xFFF5E5) }
                    Dark => { AppColors::color_from_hex(0x283641) }
                }
            }
            AppColors::Text => {
                AppColors::Background.at(color_mode.opposite())
            }
        }
    }
}



/// The different themes available.
pub enum ThemeOptions {
    Peach,
    Midnight,
    Sunrise,
    Ocean,
}
impl ThemeOptions {
    /// Gets the theme's name.
    pub fn name(&self) -> String {
        match self {
            ThemeOptions::Peach => { "Peach".to_string() }
            ThemeOptions::Midnight => { "Midnight".to_string() }
            ThemeOptions::Sunrise => { "Sunrise".to_string() }
            ThemeOptions::Ocean => { "Ocean".to_string() }
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

    /// Gets the appearance used for the theme option.
    pub fn appearance(&self) -> Appearance {
        match self {
            ThemeOptions::Peach => { Light }
            ThemeOptions::Midnight => { Dark }
            ThemeOptions::Sunrise => { Light }
            ThemeOptions::Ocean => { Dark }
        }
    }

    /// Gets the theme's background color.
    fn background(&self) -> Color {
        match self {
            ThemeOptions::Peach => { AppColors::Background.at(self.appearance()) }
            ThemeOptions::Midnight => { AppColors::Background.at(self.appearance()) }
            ThemeOptions::Sunrise => { AppColors::Salmon.at(self.appearance()) }
            ThemeOptions::Ocean => { AppColors::Periwinkle.at(self.appearance()) }
        }
    }

    /// Gets the theme's text color.
    fn text(&self) -> Color {
        match self {
            ThemeOptions::Peach => { AppColors::Text.at(self.appearance()) }
            ThemeOptions::Midnight => { AppColors::Text.at(self.appearance()) }
            ThemeOptions::Sunrise => { AppColors::Text.at(self.appearance()) }
            ThemeOptions::Ocean => { AppColors::Text.at(self.appearance()) }
        }
    }

    /// Gets the theme's primary color.
    fn primary(&self) -> Color {
        match self {
            ThemeOptions::Peach => { AppColors::Peach.at(self.appearance()) }
            ThemeOptions::Midnight => { AppColors::Powder.at(self.appearance()) }
            ThemeOptions::Sunrise => { AppColors::Honey.at(self.appearance()) }
            ThemeOptions::Ocean => { AppColors::Thistle.at(self.appearance()) }
        }
    }

    /// Gets the theme's success color.
    fn success(&self) -> Color {
        match self {
            ThemeOptions::Peach => { AppColors::Sage.at(self.appearance()) }
            ThemeOptions::Midnight => { AppColors::Sage.at(self.appearance()) }
            ThemeOptions::Sunrise => { AppColors::Sage.at(self.appearance()) }
            ThemeOptions::Ocean => { AppColors::Sage.at(self.appearance()) }
        }
    }

    /// Gets the theme's warning color.
    fn warning(&self) -> Color {
        match self {
            ThemeOptions::Peach => { AppColors::Salmon.at(self.appearance()) }
            ThemeOptions::Midnight => { AppColors::Salmon.at(self.appearance()) }
            ThemeOptions::Sunrise => { AppColors::Salmon.at(self.appearance()) }
            ThemeOptions::Ocean => { AppColors::Salmon.at(self.appearance()) }
        }
    }

    /// Gets the theme's danger color.
    fn danger(&self) -> Color {
        match self {
            ThemeOptions::Peach => { AppColors::Red.at(self.appearance()) }
            ThemeOptions::Midnight => { AppColors::Red.at(self.appearance()) }
            ThemeOptions::Sunrise => { AppColors::Red.at(self.appearance()) }
            ThemeOptions::Ocean => { AppColors::Red.at(self.appearance()) }
        }
    }
}