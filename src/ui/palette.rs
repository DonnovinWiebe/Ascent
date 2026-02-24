use std::cmp::{max, min, PartialEq};
use iced::{Color, Theme};
use iced::theme::Palette;

/// All the colors used in the application.
#[derive(Debug, Clone, Copy)]
pub enum AppColors {
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
}
impl PartialEq for AppColors {
    /// Determines if two app colors are equal.
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}
impl AppColors {
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

    /// Gets the name of the color.
    pub fn name(&self) -> String {
        match self {
            AppColors::Background => "Background".to_string(),
            AppColors::Accent => "Accent".to_string(),
            AppColors::Success => "Success".to_string(),
            AppColors::Danger => "Danger".to_string(),
            AppColors::Unavailable => "Unavailable".to_string(),
            AppColors::Text => "Text".to_string(),
            AppColors::Crimson => "Crimson".to_string(),
            AppColors::Salmon => "Salmon".to_string(),
            AppColors::Amber => "Amber".to_string(),
            AppColors::Citrus => "Citrus".to_string(),
            AppColors::Fern => "Fern".to_string(),
            AppColors::Sage => "Sage".to_string(),
            AppColors::Mint => "Mint".to_string(),
            AppColors::Teal => "Teal".to_string(),
            AppColors::Aqua => "Aqua".to_string(),
            AppColors::Sky => "Sky".to_string(),
            AppColors::Cobalt => "Cobalt".to_string(),
            AppColors::Iris => "Iris".to_string(),
            AppColors::Lavender => "Lavender".to_string(),
            AppColors::Plum => "Plum".to_string(),
            AppColors::Orchid => "Orchid".to_string(),
            AppColors::Rose => "Rose".to_string(),
        }
    }

    /// Gets the app color based on an appearance.
    pub fn themed(&self, app_theme: &AppThemes, strength: u32) -> Color {
        match self {
            AppColors::Background => {
                match app_theme {
                    AppThemes::Peach =>    { AppColors::color_from_hsl(40.00, 0.45, app_theme.lightness_for_strength(strength, AppColorStrengthTypes::Background)) }
                    AppThemes::Midnight => { AppColors::color_from_hsl(203.0, 0.30, app_theme.lightness_for_strength(strength, AppColorStrengthTypes::Background)) }
                }
            }
            AppColors::Accent => {
                match app_theme {
                    AppThemes::Peach =>    { AppColors::color_from_hsl(220.0, 0.40, app_theme.lightness_for_strength(strength, AppColorStrengthTypes::Standard)) }
                    AppThemes::Midnight => { AppColors::color_from_hsl(255.0, 0.40, app_theme.lightness_for_strength(strength, AppColorStrengthTypes::Standard)) }
                }
            }
            AppColors::Success => {
                match app_theme {
                    AppThemes::Peach =>    { AppColors::color_from_hsl(139.0, 0.46, app_theme.lightness_for_strength(strength, AppColorStrengthTypes::Standard)) }
                    AppThemes::Midnight => { AppColors::color_from_hsl(142.0, 0.40, app_theme.lightness_for_strength(strength, AppColorStrengthTypes::Standard)) }
                }
            }
            AppColors::Danger => {
                match app_theme {
                    AppThemes::Peach =>    { AppColors::color_from_hsl(358.0, 0.65, app_theme.lightness_for_strength(strength, AppColorStrengthTypes::Standard)) }
                    AppThemes::Midnight => { AppColors::color_from_hsl(356.0, 0.40, app_theme.lightness_for_strength(strength, AppColorStrengthTypes::Standard)) }
                }
            }
            AppColors::Unavailable => {
                match app_theme {
                    AppThemes::Peach =>    { AppColors::color_from_hsl(198.0, 0.15, app_theme.lightness_for_strength(strength, AppColorStrengthTypes::Standard)) }
                    AppThemes::Midnight => { AppColors::color_from_hsl(198.0, 0.15, app_theme.lightness_for_strength(strength, AppColorStrengthTypes::Standard)) }
                }
            }
            AppColors::Text => {
                match app_theme {
                    AppThemes::Peach =>    { AppColors::color_from_hsl(208.0, 0.29, app_theme.lightness_for_strength(strength, AppColorStrengthTypes::Text)) }
                    AppThemes::Midnight => { AppColors::color_from_hsl(214.0, 0.17, app_theme.lightness_for_strength(strength, AppColorStrengthTypes::Text)) }
                }
            }

            AppColors::Crimson =>          { AppColors::color_from_hsl(0.000, 0.90, app_theme.lightness_for_strength(strength, AppColorStrengthTypes::Standard)) }
            AppColors::Salmon =>           { AppColors::color_from_hsl(12.00, 1.00, app_theme.lightness_for_strength(strength, AppColorStrengthTypes::Standard)) }
            AppColors::Amber =>            { AppColors::color_from_hsl(35.00, 1.00, app_theme.lightness_for_strength(strength, AppColorStrengthTypes::Standard)) }
            AppColors::Citrus =>           { AppColors::color_from_hsl(60.00, 0.85, app_theme.lightness_for_strength(strength, AppColorStrengthTypes::Standard)) }
            AppColors::Fern =>             { AppColors::color_from_hsl(100.0, 0.55, app_theme.lightness_for_strength(strength, AppColorStrengthTypes::Standard)) }
            AppColors::Sage =>             { AppColors::color_from_hsl(135.0, 0.42, app_theme.lightness_for_strength(strength, AppColorStrengthTypes::Standard)) }
            AppColors::Mint =>             { AppColors::color_from_hsl(155.0, 0.67, app_theme.lightness_for_strength(strength, AppColorStrengthTypes::Standard)) }
            AppColors::Teal =>             { AppColors::color_from_hsl(175.0, 0.65, app_theme.lightness_for_strength(strength, AppColorStrengthTypes::Standard)) }
            AppColors::Aqua =>             { AppColors::color_from_hsl(192.0, 0.67, app_theme.lightness_for_strength(strength, AppColorStrengthTypes::Standard)) }
            AppColors::Sky =>              { AppColors::color_from_hsl(210.0, 0.67, app_theme.lightness_for_strength(strength, AppColorStrengthTypes::Standard)) }
            AppColors::Cobalt =>           { AppColors::color_from_hsl(225.0, 0.78, app_theme.lightness_for_strength(strength, AppColorStrengthTypes::Standard)) }
            AppColors::Iris =>             { AppColors::color_from_hsl(250.0, 0.75, app_theme.lightness_for_strength(strength, AppColorStrengthTypes::Standard)) }
            AppColors::Lavender =>         { AppColors::color_from_hsl(270.0, 0.65, app_theme.lightness_for_strength(strength, AppColorStrengthTypes::Standard)) }
            AppColors::Plum =>             { AppColors::color_from_hsl(285.0, 0.55, app_theme.lightness_for_strength(strength, AppColorStrengthTypes::Standard)) }
            AppColors::Orchid =>           { AppColors::color_from_hsl(315.0, 0.62, app_theme.lightness_for_strength(strength, AppColorStrengthTypes::Standard)) }
            AppColors::Rose =>             { AppColors::color_from_hsl(345.0, 0.75, app_theme.lightness_for_strength(strength, AppColorStrengthTypes::Standard)) }
        }
    }
}



/// Lists the different strength types used for different kinds of app colors.
#[derive(Debug, Clone, Copy, PartialEq)]
enum AppColorStrengthTypes {
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
    pub fn generate(&self, app_theme: &AppThemes) -> Theme {
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
    pub fn lightness_for_strength(&self, strength: u32, strength_type: AppColorStrengthTypes) -> f32 {
        if strength == 0 { panic!("App color strength cannot be 0!") }

        let mut increment = 0.05;
        let text_increment_multiplier = 3.0;
        let mut reverse_strength = false;
        let base = match strength_type {
            AppColorStrengthTypes::Background =>
                match self {
                    AppThemes::Peach =>    { 0.70 }
                    AppThemes::Midnight => { 0.10 }
                },

            AppColorStrengthTypes::Text =>
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

            AppColorStrengthTypes::Standard =>
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
        AppColors::Background.themed(self, 1)
    }

    /// Gets the theme's text color.
    fn text(&self) -> Color {
        AppColors::Text.themed(self, 1)
    }

    /// Gets the theme's primary color.
    fn primary(&self) -> Color {
        AppColors::Accent.themed(self, 1)
    }

    /// Gets the theme's success color.
    fn success(&self) -> Color {
        AppColors::Success.themed(self, 1)
    }

    /// Gets the theme's warning color.
    fn warning(&self) -> Color {
        AppColors::Unavailable.themed(self, 1)
    }

    /// Gets the theme's danger color.
    fn danger(&self) -> Color {
        AppColors::Danger.themed(self, 1)
    }
}