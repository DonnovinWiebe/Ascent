use std::cmp::{max, min, PartialEq};
use iced::{Color, Theme};
use iced::theme::Palette;

/// Defines different materials that can be used to style custom widgets.
#[derive(Debug, Clone, Copy)]
pub enum Materials {
    Plastic,
    RimmedPlastic,
    Acrylic,
}



/// All the colors used in the application.
#[derive(Debug, Clone, Copy)]
pub enum MaterialColors {
    // theming
    Background,
    Accent,
    Success,
    Danger,
    Unavailable,
    Text,

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

    // function colors
    Shadow,
}
impl PartialEq for MaterialColors {
    /// Determines if two app colors are equal.
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}
impl MaterialColors {
    /// Gets an iced color from a hex color.
    pub fn color_from_hex(hex: u32) -> Color {
        Color::from_rgb(
            ((hex >> 16) & 0xFF) as f32 / 255.0,
            ((hex >> 8) & 0xFF) as f32 / 255.0,
            (hex & 0xFF) as f32 / 255.0,
        )
    }

    /// Gets an iced color from an hsl color.
    pub fn color_from_hsl(h: f32, s: f32, l: f32) -> Color {
        // guards
        if h < 0.0 || h > 360.0 || s < 0.0 || s > 1.0 || l < 0.0 || l > 1.0 { panic!("{}", format!("Invalid HSL color: h: {:.4}, s: {:.4}, l: {:.4}", h, s, l)); }

        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = l - c / 2.0;

        let (r, g, b) = match h as u32 {
            0..=59   => (c, x, 0.0),
            60..=119 => (x, c, 0.0),
            120..=179 => (0.0, c, x),
            180..=239 => (0.0, x, c),
            240..=299 => (x, 0.0, c),
            _         => (c, 0.0, x),
        };

        Color::from_rgb(
            (r + m),
            (g + m),
            (b + m),
        )
    }

    /// Modifies the color to match a given material.
    pub fn materialized(&self, material: Materials, app_theme: &AppThemes, strength: u32) -> Color {
        match material {
            Materials::Plastic => { self.themed(app_theme, strength) }
            Materials::RimmedPlastic => { self.themed(app_theme, strength) }
            Materials::Acrylic => { Color { a: 0.85, ..self.themed(app_theme, strength) } }
        }
    }

    /// Gets the color as a shadow color.
    pub fn as_shadow(&self, app_theme: &AppThemes, strength: u32) -> Color {
        let base = self.themed(app_theme, strength);
        let darkening_multiplier = 0.35;

        Color {
            r: base.r * darkening_multiplier,
            g: base.g * darkening_multiplier,
            b: base.b * darkening_multiplier,
            a: 0.8
        }
    }

    /// Gets the name of the color.
    pub fn name(&self) -> String {
        match self {
            MaterialColors::Background => "Background".to_string(),
            MaterialColors::Accent => "Accent".to_string(),
            MaterialColors::Success => "Success".to_string(),
            MaterialColors::Danger => "Danger".to_string(),
            MaterialColors::Unavailable => "Unavailable".to_string(),
            MaterialColors::Text => "Text".to_string(),
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
            MaterialColors::Shadow => "Shadow".to_string(),
        }
    }

    /// Gets the app color based on an appearance.
    pub fn themed(&self, app_theme: &AppThemes, strength: u32) -> Color {
        match self {
            MaterialColors::Background => {
                match app_theme {
                    AppThemes::Peach =>    { MaterialColors::color_from_hsl(40.00, 0.45, app_theme.get_lightness_for_strength(strength, MaterialColorStrengthBases::Background)) }
                    AppThemes::Midnight => { MaterialColors::color_from_hsl(203.0, 0.30, app_theme.get_lightness_for_strength(strength, MaterialColorStrengthBases::Background)) }
                }
            }
            MaterialColors::Accent => {
                match app_theme {
                    AppThemes::Peach =>    { MaterialColors::color_from_hsl(220.0, 0.40, app_theme.get_lightness_for_strength(strength, MaterialColorStrengthBases::Standard)) }
                    AppThemes::Midnight => { MaterialColors::color_from_hsl(255.0, 0.40, app_theme.get_lightness_for_strength(strength, MaterialColorStrengthBases::Standard)) }
                }
            }
            MaterialColors::Success => {
                match app_theme {
                    AppThemes::Peach =>    { MaterialColors::color_from_hsl(139.0, 0.46, app_theme.get_lightness_for_strength(strength, MaterialColorStrengthBases::Standard)) }
                    AppThemes::Midnight => { MaterialColors::color_from_hsl(142.0, 0.40, app_theme.get_lightness_for_strength(strength, MaterialColorStrengthBases::Standard)) }
                }
            }
            MaterialColors::Danger => {
                match app_theme {
                    AppThemes::Peach =>    { MaterialColors::color_from_hsl(358.0, 0.65, app_theme.get_lightness_for_strength(strength, MaterialColorStrengthBases::Standard)) }
                    AppThemes::Midnight => { MaterialColors::color_from_hsl(356.0, 0.40, app_theme.get_lightness_for_strength(strength, MaterialColorStrengthBases::Standard)) }
                }
            }
            MaterialColors::Unavailable => {
                match app_theme {
                    AppThemes::Peach =>    { MaterialColors::color_from_hsl(198.0, 0.15, app_theme.get_lightness_for_strength(strength, MaterialColorStrengthBases::Standard)) }
                    AppThemes::Midnight => { MaterialColors::color_from_hsl(198.0, 0.15, app_theme.get_lightness_for_strength(strength, MaterialColorStrengthBases::Standard)) }
                }
            }
            MaterialColors::Text => {
                match app_theme {
                    AppThemes::Peach =>    { MaterialColors::color_from_hsl(208.0, 0.29, app_theme.get_lightness_for_strength(strength, MaterialColorStrengthBases::Text)) }
                    AppThemes::Midnight => { MaterialColors::color_from_hsl(214.0, 0.17, app_theme.get_lightness_for_strength(strength, MaterialColorStrengthBases::Text)) }
                }
            }

            MaterialColors::Crimson =>          { MaterialColors::color_from_hsl(0.000, 0.90, app_theme.get_lightness_for_strength(strength, MaterialColorStrengthBases::Standard)) }
            MaterialColors::Salmon =>           { MaterialColors::color_from_hsl(12.00, 1.00, app_theme.get_lightness_for_strength(strength, MaterialColorStrengthBases::Standard)) }
            MaterialColors::Amber =>            { MaterialColors::color_from_hsl(35.00, 1.00, app_theme.get_lightness_for_strength(strength, MaterialColorStrengthBases::Standard)) }
            MaterialColors::Citrus =>           { MaterialColors::color_from_hsl(60.00, 0.85, app_theme.get_lightness_for_strength(strength, MaterialColorStrengthBases::Standard)) }
            MaterialColors::Fern =>             { MaterialColors::color_from_hsl(100.0, 0.55, app_theme.get_lightness_for_strength(strength, MaterialColorStrengthBases::Standard)) }
            MaterialColors::Sage =>             { MaterialColors::color_from_hsl(135.0, 0.42, app_theme.get_lightness_for_strength(strength, MaterialColorStrengthBases::Standard)) }
            MaterialColors::Mint =>             { MaterialColors::color_from_hsl(155.0, 0.67, app_theme.get_lightness_for_strength(strength, MaterialColorStrengthBases::Standard)) }
            MaterialColors::Teal =>             { MaterialColors::color_from_hsl(175.0, 0.65, app_theme.get_lightness_for_strength(strength, MaterialColorStrengthBases::Standard)) }
            MaterialColors::Aqua =>             { MaterialColors::color_from_hsl(192.0, 0.67, app_theme.get_lightness_for_strength(strength, MaterialColorStrengthBases::Standard)) }
            MaterialColors::Sky =>              { MaterialColors::color_from_hsl(210.0, 0.67, app_theme.get_lightness_for_strength(strength, MaterialColorStrengthBases::Standard)) }
            MaterialColors::Cobalt =>           { MaterialColors::color_from_hsl(225.0, 0.78, app_theme.get_lightness_for_strength(strength, MaterialColorStrengthBases::Standard)) }
            MaterialColors::Iris =>             { MaterialColors::color_from_hsl(250.0, 0.75, app_theme.get_lightness_for_strength(strength, MaterialColorStrengthBases::Standard)) }
            MaterialColors::Lavender =>         { MaterialColors::color_from_hsl(270.0, 0.65, app_theme.get_lightness_for_strength(strength, MaterialColorStrengthBases::Standard)) }
            MaterialColors::Plum =>             { MaterialColors::color_from_hsl(285.0, 0.55, app_theme.get_lightness_for_strength(strength, MaterialColorStrengthBases::Standard)) }
            MaterialColors::Orchid =>           { MaterialColors::color_from_hsl(315.0, 0.62, app_theme.get_lightness_for_strength(strength, MaterialColorStrengthBases::Standard)) }
            MaterialColors::Rose =>             { MaterialColors::color_from_hsl(345.0, 0.75, app_theme.get_lightness_for_strength(strength, MaterialColorStrengthBases::Standard)) }

            MaterialColors::Shadow =>           { Color::BLACK }
        }
    }
}



/// Lists the different strength types used for different kinds of app colors.
#[derive(Debug, Clone, Copy, PartialEq)]
enum MaterialColorStrengthBases {
    Background,
    Text,
    Standard,
}



/// The different themes available.
#[derive(Debug, Clone)]
pub enum AppThemes {
    Peach,
    Midnight,
}
impl AppThemes {
    /// Gets the theme's name.
    pub fn name(&self) -> String {
        match self {
            AppThemes::Peach => "Peach".to_string(),
            AppThemes::Midnight => "Midnight".to_string(),
        }
    }

    /// Creates a palette for an Iced Theme.
    pub fn generate_iced_palette(&self, app_theme: &AppThemes) -> Theme {
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

    /// Gets the standard lightness for an app color at a given strength.
    /// Strength starts at 1.
    pub fn get_lightness_for_strength(&self, strength: u32, strength_type: MaterialColorStrengthBases) -> f32 {
        if strength == 0 { panic!("App color strength cannot be 0!") }

        let mut increment = 0.05;
        let text_increment_multiplier = 3.0;
        let mut reverse_strength = false;
        let base = match strength_type {
            MaterialColorStrengthBases::Background =>
                match self {
                    AppThemes::Peach =>    { 0.70 }
                    AppThemes::Midnight => { 0.10 }
                },

            MaterialColorStrengthBases::Text =>
                match self {
                    AppThemes::Peach =>    {
                        increment = increment * text_increment_multiplier;
                        0.10
                    }
                    AppThemes::Midnight => {
                        increment = increment * text_increment_multiplier;
                        reverse_strength = true;
                        0.90
                    }
                },

            MaterialColorStrengthBases::Standard =>
                match self {
                    AppThemes::Peach =>    { 0.65 }
                    AppThemes::Midnight => { 0.25 }
                },
        };

        if reverse_strength {
            (base - (increment * (strength - 1) as f64)).max(0.0) as f32
        }
        else {
            (base + (increment * (strength - 1) as f64)).min(1.0) as f32
        }
    }

    /// Gets the theme's background color.
    fn background(&self) -> Color {
        MaterialColors::Background.themed(self, 1)
    }

    /// Gets the theme's text color.
    fn text(&self) -> Color {
        MaterialColors::Text.themed(self, 1)
    }

    /// Gets the theme's primary color.
    fn primary(&self) -> Color {
        MaterialColors::Accent.themed(self, 1)
    }

    /// Gets the theme's success color.
    fn success(&self) -> Color {
        MaterialColors::Success.themed(self, 1)
    }

    /// Gets the theme's warning color.
    fn warning(&self) -> Color {
        MaterialColors::Unavailable.themed(self, 1)
    }

    /// Gets the theme's danger color.
    fn danger(&self) -> Color {
        MaterialColors::Danger.themed(self, 1)
    }
}