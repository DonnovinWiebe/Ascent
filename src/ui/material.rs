use std::cmp::PartialEq;
use iced::{Color, Theme};
use iced::theme::Palette;

/// Allows Widgets to be created with a full material style in a single parameter instead of several parameters.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MaterialStyle {
    pub material: Materials,
    pub color: MaterialColors,
    pub depth: Depths,
}
impl MaterialStyle {
    /// Returns if this style casts a shadow.
    #[must_use]
    pub fn casts_shadow(self) -> bool {
        self.depth != Depths::Flat
    }
}



/// Defines different materials that can be used to style custom widgets.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Materials {
    Plastic,
    Acrylic,
}



/// Defines the the height options for custom widgets.
/// `Flat` means that the content is flat with its surroundings (no shadow).
/// `Proud` means the the content is raised above its surroundings.
/// `Recessed` means the content is lowered into its surroundings.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Depths {
    Flat,
    Proud,
    Recessed,
}



/// All the colors used in the application.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum MaterialColors {
    // theming
    Background,
    Card,
    CardContent,
    CardHollow,
    CardHollowContent,
    Unavailable,
    StrongText,
    MediumText,
    WeakText,

    // standard colors
    Crimson,
    Salmon,
    Amber,
    Citrus,
    Fern,
    Sage,
    Mint,
    Teal,
    Aqua,
    Sky,
    Cobalt,
    Iris,
    Lavender,
    Plum,
    Orchid,
    Rose,
}
impl MaterialColors {
    /// Gets an iced color from a hex color.
    #[must_use]
    pub fn color_from_hex(hex: u32) -> Color {
        Color::from_rgb(
            f32::from(((hex >> 16) & 0xFF) as u8) / 255.0,
            f32::from(((hex >> 8) & 0xFF) as u8) / 255.0,
            f32::from((hex & 0xFF) as u8) / 255.0,
        )
    }

    /// Gets an iced color from an hsl color.
    /// # Panics
    /// Panics if h is not in the range 0..360, or s or l are not in the range 0..=1.
    #[must_use]
    pub fn color_from_hsl(hue: f32, saturation: f32, lightness: f32) -> Color {
        assert!((0.0..360.0).contains(&hue) && (0.0..=1.0).contains(&saturation) && (0.0..=1.0).contains(&lightness), "Invalid HSL color: h: {hue:.4}, s: {saturation:.4}, l: {lightness:.4}");

        let chroma = (1.0 - (2.0 * lightness - 1.0).abs()) * saturation;
        let x = chroma * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs());
        let offset = lightness - chroma / 2.0;

        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // h is guaranteed to be in the range 0..360
        let (r, g, b) = match hue as u32 {
            0..=59    => (chroma, x, 0.0),
            60..=119  => (x, chroma, 0.0),
            120..=179 => (0.0, chroma, x),
            180..=239 => (0.0, x, chroma),
            240..=299 => (x, 0.0, chroma),
            _         => (chroma, 0.0, x),
        };

        Color::from_rgb(
            r + offset,
            g + offset,
            b + offset,
        )
    }

    /// Gets the name of the color.
    #[must_use]
    pub fn name(self) -> String {
        match self {
            // theming
            MaterialColors::Background => "Background".to_string(),
            MaterialColors::Card => "Card".to_string(),
            MaterialColors::CardContent => "Card Content".to_string(),
            MaterialColors::CardHollow => "Card Hollow".to_string(),
            MaterialColors::CardHollowContent => "Card Hollow Content".to_string(),
            MaterialColors::Unavailable => "Unavailable".to_string(),
            MaterialColors::StrongText => "Strong Text".to_string(),
            MaterialColors::MediumText => "Medium Text".to_string(),
            MaterialColors::WeakText => "Weak Text".to_string(),
            
            // standard colors
            MaterialColors::Crimson => "Crimson".to_string(),
            MaterialColors::Salmon => "Salmon".to_string(),
            MaterialColors::Amber => "Amber".to_string(),
            MaterialColors::Citrus => "Citrus".to_string(),
            MaterialColors::Fern => "Fern".to_string(),
            MaterialColors::Sage => "Sage".to_string(),
            MaterialColors::Mint => "Mint".to_string(),
            MaterialColors::Teal => "Teal".to_string(),
            MaterialColors::Aqua => "Aqua".to_string(),
            MaterialColors::Sky => "Sky".to_string(),
            MaterialColors::Cobalt => "Cobalt".to_string(),
            MaterialColors::Iris => "Iris".to_string(),
            MaterialColors::Lavender => "Lavender".to_string(),
            MaterialColors::Plum => "Plum".to_string(),
            MaterialColors::Orchid => "Orchid".to_string(),
            MaterialColors::Rose => "Rose".to_string(),
        }
    }
    
    /// Gets the accent color for the given theme.
    #[must_use]
    pub fn accent(theme: AppThemes) -> MaterialColors {
        match theme {
            AppThemes::Peach => MaterialColors::Salmon,
            AppThemes::Midnight => MaterialColors::Orchid,
            AppThemes::Sunrise => MaterialColors::Citrus,
            AppThemes::DarkForest => MaterialColors::Mint,
        }
    }

    /// Gets a usable `Color` from the given material color.
    #[must_use]
    pub fn materialized(self, material: Materials, depth: Depths, is_shadow: bool, app_theme: AppThemes) -> Color {
        let alpha = match material {
            Materials::Plastic => 1.0,
            Materials::Acrylic => 0.9,
        };
        let mut shadow_modifier = 0.0;
        if is_shadow {
            shadow_modifier = match depth {
                Depths::Flat => 0.0,
                Depths::Proud => -0.08,
                Depths::Recessed => 0.06,
            };
        }
        
        #[allow(clippy::match_same_arms)] // This just makes color theming more ergonomic to inspect and/or change later.
        match self {
            // theming colors
            MaterialColors::Background => {
                match app_theme {
                    AppThemes::Peach      => Color { a: alpha, ..MaterialColors::color_from_hsl(040.0, 0.50, 0.80 + shadow_modifier) },
                    AppThemes::Sunrise    => Color { a: alpha, ..MaterialColors::color_from_hsl(028.0, 0.70, 0.80 + shadow_modifier) },
                    AppThemes::Midnight   => Color {a: alpha, .. MaterialColors::color_from_hsl(203.0, 0.30, 0.12 + shadow_modifier) },
                    AppThemes::DarkForest => Color { a: alpha, ..MaterialColors::color_from_hsl(100.0, 0.28, 0.15 + shadow_modifier) },
                }
            }
            MaterialColors::Card => {
                match app_theme {
                    AppThemes::Peach      => Color { a: alpha, ..MaterialColors::color_from_hsl(040.0, 0.40, 0.70 + shadow_modifier) },
                    AppThemes::Sunrise    => Color { a: alpha, ..MaterialColors::color_from_hsl(022.0, 0.55, 0.70 + shadow_modifier) },
                    AppThemes::Midnight   => Color { a: alpha, ..MaterialColors::color_from_hsl(203.0, 0.28, 0.22 + shadow_modifier) },
                    AppThemes::DarkForest => Color { a: alpha, ..MaterialColors::color_from_hsl(095.0, 0.25, 0.45 + shadow_modifier) },
                }
            }
            MaterialColors::CardContent => {
                match app_theme {
                    AppThemes::Peach      => Color { a: alpha, ..MaterialColors::color_from_hsl(160.0, 0.30, 0.70 + shadow_modifier) },
                    AppThemes::Sunrise    => Color { a: alpha, ..MaterialColors::color_from_hsl(340.0, 0.28, 0.70 + shadow_modifier) },
                    AppThemes::Midnight   => Color { a: alpha, ..MaterialColors::color_from_hsl(230.0, 0.25, 0.29 + shadow_modifier) },
                    AppThemes::DarkForest => Color { a: alpha, ..MaterialColors::color_from_hsl(075.0, 0.30, 0.45 + shadow_modifier) },
                }
            }
            MaterialColors::CardHollow => {
                // darkened card
                match app_theme {
                    AppThemes::Peach      => Color { a: alpha, ..MaterialColors::color_from_hsl(040.0, 0.40, 0.55 + shadow_modifier) },
                    AppThemes::Sunrise    => Color { a: alpha, ..MaterialColors::color_from_hsl(022.0, 0.55, 0.55 + shadow_modifier) },
                    AppThemes::Midnight   => Color { a: alpha, ..MaterialColors::color_from_hsl(203.0, 0.28, 0.15 + shadow_modifier) },
                    AppThemes::DarkForest => Color { a: alpha, ..MaterialColors::color_from_hsl(095.0, 0.25, 0.25 + shadow_modifier) },
                }
            }
            MaterialColors::CardHollowContent => {
                // darkened card content
                match app_theme {
                    AppThemes::Peach      => Color { a: alpha, ..MaterialColors::color_from_hsl(160.0, 0.30, 0.60 + shadow_modifier) },
                    AppThemes::Sunrise    => Color { a: alpha, ..MaterialColors::color_from_hsl(340.0, 0.28, 0.60 + shadow_modifier) },
                    AppThemes::Midnight   => Color { a: alpha, ..MaterialColors::color_from_hsl(230.0, 0.25, 0.22 + shadow_modifier) },
                    AppThemes::DarkForest => Color { a: alpha, ..MaterialColors::color_from_hsl(075.0, 0.30, 0.25 + shadow_modifier) },
                }
            }
            MaterialColors::Unavailable => {
                match app_theme {
                    AppThemes::Peach      => Color { a: alpha, ..MaterialColors::color_from_hsl(200.0, 0.18, 0.65 + shadow_modifier) },
                    AppThemes::Sunrise    => Color { a: alpha, ..MaterialColors::color_from_hsl(025.0, 0.15, 0.65 + shadow_modifier) },
                    AppThemes::Midnight   => Color { a: alpha, ..MaterialColors::color_from_hsl(160.0, 0.10, 0.35 + shadow_modifier) },
                    AppThemes::DarkForest => Color { a: alpha, ..MaterialColors::color_from_hsl(100.0, 0.10, 0.35 + shadow_modifier) },
                }
            }
            MaterialColors::StrongText => {
                match app_theme {
                    AppThemes::Peach      => Color { a: alpha, ..MaterialColors::color_from_hsl(208.0, 0.29, 0.10 + shadow_modifier) },
                    AppThemes::Sunrise    => Color { a: alpha, ..MaterialColors::color_from_hsl(020.0, 0.35, 0.10 + shadow_modifier) },
                    AppThemes::Midnight   => Color { a: alpha, ..MaterialColors::color_from_hsl(214.0, 0.17, 0.95 + shadow_modifier) },
                    AppThemes::DarkForest => Color { a: alpha, ..MaterialColors::color_from_hsl(080.0, 0.40, 0.95 + shadow_modifier) },
                }
            }
            MaterialColors::MediumText => {
                match app_theme {
                    AppThemes::Peach      => Color { a: alpha, ..MaterialColors::color_from_hsl(208.0, 0.29, 0.25 + shadow_modifier) },
                    AppThemes::Sunrise    => Color { a: alpha, ..MaterialColors::color_from_hsl(020.0, 0.30, 0.25 + shadow_modifier) },
                    AppThemes::Midnight   => Color { a: alpha, ..MaterialColors::color_from_hsl(214.0, 0.17, 0.80 + shadow_modifier) },
                    AppThemes::DarkForest => Color { a: alpha, ..MaterialColors::color_from_hsl(080.0, 0.30, 0.80 + shadow_modifier) },
                }
            }
            MaterialColors::WeakText => {
                match app_theme {
                    AppThemes::Peach      => Color { a: alpha, ..MaterialColors::color_from_hsl(208.0, 0.29, 0.40 + shadow_modifier) },
                    AppThemes::Sunrise    => Color { a: alpha, ..MaterialColors::color_from_hsl(020.0, 0.22, 0.40 + shadow_modifier) },
                    AppThemes::Midnight   => Color { a: alpha, ..MaterialColors::color_from_hsl(214.0, 0.17, 0.65 + shadow_modifier) },
                    AppThemes::DarkForest => Color { a: alpha, ..MaterialColors::color_from_hsl(080.0, 0.20, 0.65 + shadow_modifier) },
                }
            }

            // standard colors
            MaterialColors::Crimson => {
                match app_theme {
                    AppThemes::Peach      => Color { a: alpha, ..MaterialColors::color_from_hsl(000.0, 0.90, 0.70 + shadow_modifier) },
                    AppThemes::Sunrise    => Color { a: alpha, ..MaterialColors::color_from_hsl(000.0, 0.90, 0.70 + shadow_modifier) },
                    AppThemes::Midnight   => Color { a: alpha, ..MaterialColors::color_from_hsl(000.0, 0.90, 0.28 + shadow_modifier) },
                    AppThemes::DarkForest => Color { a: alpha, ..MaterialColors::color_from_hsl(000.0, 0.90, 0.28 + shadow_modifier) },
                }
            }
            MaterialColors::Salmon => {
                match app_theme {
                    AppThemes::Peach      => Color { a: alpha, ..MaterialColors::color_from_hsl(012.0, 1.00, 0.70 + shadow_modifier) },
                    AppThemes::Sunrise    => Color { a: alpha, ..MaterialColors::color_from_hsl(012.0, 1.00, 0.70 + shadow_modifier) },
                    AppThemes::Midnight   => Color { a: alpha, ..MaterialColors::color_from_hsl(012.0, 1.00, 0.28 + shadow_modifier) },
                    AppThemes::DarkForest => Color { a: alpha, ..MaterialColors::color_from_hsl(012.0, 1.00, 0.28 + shadow_modifier) },
                }
            }
            MaterialColors::Amber => {
                match app_theme {
                    AppThemes::Peach      => Color { a: alpha, ..MaterialColors::color_from_hsl(035.0, 1.00, 0.70 + shadow_modifier) },
                    AppThemes::Sunrise    => Color { a: alpha, ..MaterialColors::color_from_hsl(035.0, 1.00, 0.70 + shadow_modifier) },
                    AppThemes::Midnight   => Color { a: alpha, ..MaterialColors::color_from_hsl(035.0, 1.00, 0.28 + shadow_modifier) },
                    AppThemes::DarkForest => Color { a: alpha, ..MaterialColors::color_from_hsl(035.0, 1.00, 0.28 + shadow_modifier) },
                }
            }
            MaterialColors::Citrus => {
                
                match app_theme {
                    AppThemes::Peach      => Color { a: alpha, ..MaterialColors::color_from_hsl(060.0, 0.85, 0.70 + shadow_modifier) },
                    AppThemes::Sunrise    => Color { a: alpha, ..MaterialColors::color_from_hsl(060.0, 0.85, 0.70 + shadow_modifier) },
                    AppThemes::Midnight   => Color { a: alpha, ..MaterialColors::color_from_hsl(060.0, 0.85, 0.28 + shadow_modifier) },
                    AppThemes::DarkForest => Color { a: alpha, ..MaterialColors::color_from_hsl(060.0, 0.85, 0.28 + shadow_modifier) },
                }
            }
            MaterialColors::Fern => {
                match app_theme {
                    AppThemes::Peach      => Color { a: alpha, ..MaterialColors::color_from_hsl(100.0, 0.55, 0.70 + shadow_modifier) },
                    AppThemes::Sunrise    => Color { a: alpha, ..MaterialColors::color_from_hsl(100.0, 0.55, 0.70 + shadow_modifier) },
                    AppThemes::Midnight   => Color { a: alpha, ..MaterialColors::color_from_hsl(100.0, 0.55, 0.28 + shadow_modifier) },
                    AppThemes::DarkForest => Color { a: alpha, ..MaterialColors::color_from_hsl(100.0, 0.55, 0.28 + shadow_modifier) },
                }
            }
            MaterialColors::Sage => {
                match app_theme {
                    AppThemes::Peach      => Color { a: alpha, ..MaterialColors::color_from_hsl(135.0, 0.42, 0.70 + shadow_modifier) },
                    AppThemes::Sunrise    => Color { a: alpha, ..MaterialColors::color_from_hsl(135.0, 0.42, 0.65 + shadow_modifier) },
                    AppThemes::Midnight   => Color { a: alpha, ..MaterialColors::color_from_hsl(135.0, 0.42, 0.28 + shadow_modifier) },
                    AppThemes::DarkForest => Color { a: alpha, ..MaterialColors::color_from_hsl(135.0, 0.42, 0.28 + shadow_modifier) },
                }
            }
            MaterialColors::Mint => {
                match app_theme {
                    AppThemes::Peach      => Color { a: alpha, ..MaterialColors::color_from_hsl(155.0, 0.67, 0.70 + shadow_modifier) },
                    AppThemes::Sunrise    => Color { a: alpha, ..MaterialColors::color_from_hsl(155.0, 0.67, 0.70 + shadow_modifier) },
                    AppThemes::Midnight   => Color { a: alpha, ..MaterialColors::color_from_hsl(155.0, 0.67, 0.28 + shadow_modifier) },
                    AppThemes::DarkForest => Color { a: alpha, ..MaterialColors::color_from_hsl(155.0, 0.67, 0.28 + shadow_modifier) },
                }
            }
            MaterialColors::Teal => {
                match app_theme {
                    AppThemes::Peach      => Color { a: alpha, ..MaterialColors::color_from_hsl(175.0, 0.65, 0.70 + shadow_modifier) },
                    AppThemes::Sunrise    => Color { a: alpha, ..MaterialColors::color_from_hsl(175.0, 0.65, 0.70 + shadow_modifier) },
                    AppThemes::Midnight   => Color { a: alpha, ..MaterialColors::color_from_hsl(175.0, 0.65, 0.28 + shadow_modifier) },
                    AppThemes::DarkForest => Color { a: alpha, ..MaterialColors::color_from_hsl(175.0, 0.65, 0.28 + shadow_modifier) },
                }
            }
            MaterialColors::Aqua => {
                match app_theme {
                    AppThemes::Peach      => Color { a: alpha, ..MaterialColors::color_from_hsl(192.0, 0.67, 0.70 + shadow_modifier) },
                    AppThemes::Sunrise    => Color { a: alpha, ..MaterialColors::color_from_hsl(192.0, 0.67, 0.70 + shadow_modifier) },
                    AppThemes::Midnight   => Color { a: alpha, ..MaterialColors::color_from_hsl(192.0, 0.67, 0.28 + shadow_modifier) },
                    AppThemes::DarkForest => Color { a: alpha, ..MaterialColors::color_from_hsl(192.0, 0.67, 0.28 + shadow_modifier) },
                }
            }
            MaterialColors::Sky => {
                match app_theme {
                    AppThemes::Peach      => Color { a: alpha, ..MaterialColors::color_from_hsl(210.0, 0.67, 0.70 + shadow_modifier) },
                    AppThemes::Sunrise    => Color { a: alpha, ..MaterialColors::color_from_hsl(210.0, 0.67, 0.70 + shadow_modifier) },
                    AppThemes::Midnight   => Color { a: alpha, ..MaterialColors::color_from_hsl(210.0, 0.67, 0.28 + shadow_modifier) },
                    AppThemes::DarkForest => Color { a: alpha, ..MaterialColors::color_from_hsl(210.0, 0.67, 0.28 + shadow_modifier) },
                }
            }
            MaterialColors::Cobalt => {
                match app_theme {
                    AppThemes::Peach      => Color { a: alpha, ..MaterialColors::color_from_hsl(225.0, 0.78, 0.70 + shadow_modifier) },
                    AppThemes::Sunrise    => Color { a: alpha, ..MaterialColors::color_from_hsl(225.0, 0.78, 0.70 + shadow_modifier) },
                    AppThemes::Midnight   => Color { a: alpha, ..MaterialColors::color_from_hsl(225.0, 0.78, 0.28 + shadow_modifier) },
                    AppThemes::DarkForest => Color { a: alpha, ..MaterialColors::color_from_hsl(225.0, 0.78, 0.28 + shadow_modifier) },
                }
            }
            MaterialColors::Iris => {
                match app_theme {
                    AppThemes::Peach      => Color { a: alpha, ..MaterialColors::color_from_hsl(250.0, 0.75, 0.70 + shadow_modifier) },
                    AppThemes::Sunrise    => Color { a: alpha, ..MaterialColors::color_from_hsl(250.0, 0.75, 0.70 + shadow_modifier) },
                    AppThemes::Midnight   => Color { a: alpha, ..MaterialColors::color_from_hsl(250.0, 0.75, 0.28 + shadow_modifier) },
                    AppThemes::DarkForest => Color { a: alpha, ..MaterialColors::color_from_hsl(250.0, 0.75, 0.28 + shadow_modifier) },
                }
            }
            MaterialColors::Lavender => {
                match app_theme {
                    AppThemes::Peach      => Color { a: alpha, ..MaterialColors::color_from_hsl(270.0, 0.65, 0.70 + shadow_modifier) },
                    AppThemes::Sunrise    => Color { a: alpha, ..MaterialColors::color_from_hsl(270.0, 0.65, 0.70 + shadow_modifier) },
                    AppThemes::Midnight   => Color { a: alpha, ..MaterialColors::color_from_hsl(270.0, 0.65, 0.28 + shadow_modifier) },
                    AppThemes::DarkForest => Color { a: alpha, ..MaterialColors::color_from_hsl(270.0, 0.65, 0.28 + shadow_modifier) },
                }
            }
            MaterialColors::Plum => {
                match app_theme {
                    AppThemes::Peach      => Color { a: alpha, ..MaterialColors::color_from_hsl(285.0, 0.55, 0.70 + shadow_modifier) },
                    AppThemes::Sunrise    => Color { a: alpha, ..MaterialColors::color_from_hsl(285.0, 0.55, 0.70 + shadow_modifier) },
                    AppThemes::Midnight   => Color { a: alpha, ..MaterialColors::color_from_hsl(285.0, 0.55, 0.28 + shadow_modifier) },
                    AppThemes::DarkForest => Color { a: alpha, ..MaterialColors::color_from_hsl(285.0, 0.55, 0.28 + shadow_modifier) },
                }
            }
            MaterialColors::Orchid => {
                match app_theme {
                    AppThemes::Peach      => Color { a: alpha, ..MaterialColors::color_from_hsl(315.0, 0.62, 0.70 + shadow_modifier) },
                    AppThemes::Sunrise    => Color { a: alpha, ..MaterialColors::color_from_hsl(315.0, 0.62, 0.70 + shadow_modifier) },
                    AppThemes::Midnight   => Color { a: alpha, ..MaterialColors::color_from_hsl(315.0, 0.62, 0.28 + shadow_modifier) },
                    AppThemes::DarkForest => Color { a: alpha, ..MaterialColors::color_from_hsl(315.0, 0.62, 0.28 + shadow_modifier) },
                }
            }
            MaterialColors::Rose => {
                match app_theme {
                    AppThemes::Peach      => Color { a: alpha, ..MaterialColors::color_from_hsl(345.0, 0.75, 0.70 + shadow_modifier) },
                    AppThemes::Sunrise    => Color { a: alpha, ..MaterialColors::color_from_hsl(345.0, 0.75, 0.70 + shadow_modifier) },
                    AppThemes::Midnight   => Color { a: alpha, ..MaterialColors::color_from_hsl(345.0, 0.75, 0.28 + shadow_modifier) },
                    AppThemes::DarkForest => Color { a: alpha, ..MaterialColors::color_from_hsl(345.0, 0.75, 0.28 + shadow_modifier) },
                }
            }
        }
    }
    
    /// Gets the list of standard colors.
    #[must_use]
    pub fn standard_colors() -> Vec<MaterialColors> {
        vec![
            MaterialColors::Crimson,
            MaterialColors::Salmon,
            MaterialColors::Amber,
            MaterialColors::Citrus,
            MaterialColors::Fern,
            MaterialColors::Sage,
            MaterialColors::Mint,
            MaterialColors::Teal,
            MaterialColors::Aqua,
            MaterialColors::Sky,
            MaterialColors::Cobalt,
            MaterialColors::Iris,
            MaterialColors::Lavender,
            MaterialColors::Plum,
            MaterialColors::Orchid,
            MaterialColors::Rose,
        ]
    }
    
    /// Returns a random standard color.
    #[must_use]
    pub fn random() -> MaterialColors {
        use rand::prelude::*;
        
        let mut rng = rand::rng();
        let colors = MaterialColors::standard_colors();
        let random_index_result = (0..colors.len()).choose(&mut rng);
        let random_index = random_index_result.unwrap_or(0);
        colors[random_index]
    }
    
    
    
    // Iced theme color shorthand functions
    /// Gets the theme's background color.
    #[must_use]
    pub fn background() -> MaterialColors {
        MaterialColors::Background
    }

    /// Gets the theme's text color.
    #[must_use]
    pub fn text() -> MaterialColors {
        MaterialColors::StrongText
    }

    /// Gets the theme's primary color.
    #[must_use]
    pub fn primary() -> MaterialColors {
        MaterialColors::CardContent
    }

    /// Gets the theme's success color.
    #[must_use]
    pub fn success() -> MaterialColors {
        MaterialColors::Fern
    }

    /// Gets the theme's warning color.
    #[must_use]
    pub fn warning() -> MaterialColors {
        MaterialColors::Amber
    }

    /// Gets the theme's danger color.
    #[must_use]
    pub fn danger() -> MaterialColors {
        MaterialColors::Crimson
    }
}



/// The different themes available.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AppThemes {
    Peach,
    Sunrise,
    Midnight,
    DarkForest,
}
impl AppThemes {
    /// Gets the theme's name.
    #[must_use]
    pub fn name(&self) -> String {
        match self {
            AppThemes::Peach => "Peach".to_string(),
            AppThemes::Midnight => "Midnight".to_string(),
            AppThemes::Sunrise => "Sunrise".to_string(),
            AppThemes::DarkForest => "Dark Forest".to_string(),
        }
    }

    /// Creates a palette for an Iced Theme.
    #[must_use]
    pub fn generate_iced_palette(&self) -> Theme {
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

    /// Gets the theme's background color.
    #[must_use]
    fn background(self) -> Color {
        MaterialColors::background().materialized(Materials::Plastic, Depths::Flat, false, self)
    }

    /// Gets the theme's text color.
    #[must_use]
    fn text(self) -> Color {
        MaterialColors::text().materialized(Materials::Plastic, Depths::Flat, false, self)
    }

    /// Gets the theme's primary color.
    #[must_use]
    fn primary(self) -> Color {
        MaterialColors::primary().materialized(Materials::Plastic, Depths::Flat, false, self)
    }

    /// Gets the theme's success color.
    #[must_use]
    fn success(self) -> Color {
        MaterialColors::success().materialized(Materials::Plastic, Depths::Flat, false, self)
    }

    /// Gets the theme's warning color.
    #[must_use]
    fn warning(self) -> Color {
        MaterialColors::warning().materialized(Materials::Plastic, Depths::Flat, false, self)
    }

    /// Gets the theme's danger color.
    #[must_use]
    fn danger(self) -> Color {
        MaterialColors::danger().materialized(Materials::Plastic, Depths::Flat, false, self)
    }
}