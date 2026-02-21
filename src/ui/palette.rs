use std::cmp::PartialEq;
use iced::{Color, Theme};
use iced::theme::Palette;

/// The color variations available per App Color.
pub enum AppColorStrengths {
    Base,
    Secondary,
}



/// All the colors used in the application.
#[derive(Debug, Clone, Copy)]
pub enum AppColors {
    // theming
    Background,
    Midground,
    Foreground,
    Accent,
    Success,
    Danger,
    Unavailable,
    Text,

    // colors
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
}
impl PartialEq for AppColors {
    /// Determines if two app colors are equal.
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
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

    /// Gets the name of the color.
    pub fn name(&self) -> String {
        match self {
            AppColors::Background => "Background".to_string(),
            AppColors::Midground => "Midground".to_string(),
            AppColors::Foreground => "Foreground".to_string(),
            AppColors::Accent => "Accent".to_string(),
            AppColors::Success => "Success".to_string(),
            AppColors::Danger => "Danger".to_string(),
            AppColors::Unavailable => "Unavailable".to_string(),
            AppColors::Text => "Text".to_string(),
            AppColors::Amber => "Amber".to_string(),
            AppColors::Apricot => "Apricot".to_string(),
            AppColors::Aqua => "Aqua".to_string(),
            AppColors::Blush => "Blush".to_string(),
            AppColors::Butter => "Butter".to_string(),
            AppColors::Honey => "Honey".to_string(),
            AppColors::Lavender => "Lavender".to_string(),
            AppColors::Lilac => "Lilac".to_string(),
            AppColors::Mauve => "Mauve".to_string(),
            AppColors::Mint => "Mint".to_string(),
            AppColors::Orchid => "Orchid".to_string(),
            AppColors::Peach => "Peach".to_string(),
            AppColors::Periwinkle => "Periwinkle".to_string(),
            AppColors::Plum => "Plum".to_string(),
            AppColors::Powder => "Powder".to_string(),
            AppColors::Red => "Red".to_string(),
            AppColors::Rose => "Rose".to_string(),
            AppColors::Sage => "Sage".to_string(),
            AppColors::Salmon => "Salmon".to_string(),
            AppColors::Seafoam => "Seafoam".to_string(),
            AppColors::Sky => "Sky".to_string(),
            AppColors::Thistle => "Thistle".to_string(),
        }
    }

    /// Gets the app color based on an appearance.
    pub fn themed(&self, app_theme: &AppThemes, strength: AppColorStrengths) -> Color {
        match self {
            AppColors::Background => {
                match app_theme {
                    AppThemes::Peach => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0xe0c285) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0xe6cc99) }
                        }
                    }
                    AppThemes::Midnight => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0x152128) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0x1b2932) }
                        }
                    }
                }
            }

            AppColors::Midground => {
                match app_theme {
                    AppThemes::Peach => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0xebd6ad) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0xf0e0c2) }
                        }
                    }
                    AppThemes::Midnight => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0x243742) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0x2d4453) }
                        }
                    }
                }
            }

            AppColors::Foreground => {
                match app_theme {
                    AppThemes::Peach => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0xf5ebd6) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0xfaf5eb) }
                        }
                    }
                    AppThemes::Midnight => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0x365263) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0x3e5f74) }
                        }
                    }
                }
            }

            AppColors::Accent => {
                match app_theme {
                    AppThemes::Peach => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0x829ac9) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0x94a8d1) }
                        }
                    }
                    AppThemes::Midnight => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0x5c45a1) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0x664db2) }
                        }
                    }
                }
            }

            AppColors::Success => {
                match app_theme {
                    AppThemes::Peach => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0x7dcf97) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0x8fd6a6) }
                        }
                    }
                    AppThemes::Midnight => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0x45a167) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0x4db272) }
                        }
                    }
                }
            }

            AppColors::Danger => {
                match app_theme {
                    AppThemes::Peach => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0xe06c70) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0xe48184) }
                        }
                    }
                    AppThemes::Midnight => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0xa1454b) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0xb24d53) }
                        }
                    }
                }
            }

            AppColors::Unavailable => {
                match app_theme {
                    AppThemes::Peach => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0x6c8793) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0x98abb3) }
                        }
                    }
                    AppThemes::Midnight => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0x415158) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0x4c5f67) }
                        }
                    }
                }
            }

            AppColors::Text => {
                match app_theme {
                    AppThemes::Peach => {
                        match strength {
                            AppColorStrengths::Base => { AppColors::color_from_hex(0x121a21) }
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0x364e63) }
                        }
                    }
                    AppThemes::Midnight => {
                        match strength {
                            AppColorStrengths::Base => { AppColors::color_from_hex(0xe1e5ea) }
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0xa5b1c0) }
                        }
                    }
                }
            }

            AppColors::Amber => {
                match app_theme {
                    AppThemes::Peach => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0xFFF4B8) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0xFFF4B8) }
                        }
                    }
                    AppThemes::Midnight => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0xCCB85C) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0xCCB85C) }
                        }
                    }
                }
            }

            AppColors::Apricot => {
                match app_theme {
                    AppThemes::Peach => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0xFFE6B8) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0xFFE6B8) }
                        }
                    }
                    AppThemes::Midnight => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0xCC9B5C) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0xCC9B5C) }
                        }
                    }
                }
            }

            AppColors::Aqua => {
                match app_theme {
                    AppThemes::Peach => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0xB8E6F1) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0xB8E6F1) }
                        }
                    }
                    AppThemes::Midnight => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0x5C9BB8) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0x5C9BB8) }
                        }
                    }
                }
            }

            AppColors::Blush => {
                match app_theme {
                    AppThemes::Peach => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0xF9D4E6) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0xF9D4E6) }
                        }
                    }
                    AppThemes::Midnight => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0xB87B9B) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0xB87B9B) }
                        }
                    }
                }
            }

            AppColors::Butter => {
                match app_theme {
                    AppThemes::Peach => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0xFFF1D4) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0xFFF1D4) }
                        }
                    }
                    AppThemes::Midnight => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0xCCAD7B) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0xCCAD7B) }
                        }
                    }
                }
            }

            AppColors::Honey => {
                match app_theme {
                    AppThemes::Peach => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0xF9E6C5) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0xF9E6C5) }
                        }
                    }
                    AppThemes::Midnight => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0xB89B6B) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0xB89B6B) }
                        }
                    }
                }
            }

            AppColors::Lavender => {
                match app_theme {
                    AppThemes::Peach => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0xD4C5F9) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0xD4C5F9) }
                        }
                    }
                    AppThemes::Midnight => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0x7B5FB8) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0x7B5FB8) }
                        }
                    }
                }
            }

            AppColors::Lilac => {
                match app_theme {
                    AppThemes::Peach => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0xE6D4F1) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0xE6D4F1) }
                        }
                    }
                    AppThemes::Midnight => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0x9B7BB8) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0x9B7BB8) }
                        }
                    }
                }
            }

            AppColors::Mauve => {
                match app_theme {
                    AppThemes::Peach => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0xE6B8D4) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0xE6B8D4) }
                        }
                    }
                    AppThemes::Midnight => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0x9B5F7B) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0x9B5F7B) }
                        }
                    }
                }
            }

            AppColors::Mint => {
                match app_theme {
                    AppThemes::Peach => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0xB8F1D4) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0xB8F1D4) }
                        }
                    }
                    AppThemes::Midnight => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0x4AA77B) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0x4AA77B) }
                        }
                    }
                }
            }

            AppColors::Orchid => {
                match app_theme {
                    AppThemes::Peach => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0xF1C5E6) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0xF1C5E6) }
                        }
                    }
                    AppThemes::Midnight => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0xB86B9B) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0xB86B9B) }
                        }
                    }
                }
            }

            AppColors::Peach => {
                match app_theme {
                    AppThemes::Peach => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0xFFD4B8) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0xFFD4B8) }
                        }
                    }
                    AppThemes::Midnight => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0xCC8A5C) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0xCC8A5C) }
                        }
                    }
                }
            }

            AppColors::Periwinkle => {
                match app_theme {
                    AppThemes::Peach => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0xC5D4F9) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0xC5D4F9) }
                        }
                    }
                    AppThemes::Midnight => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0x6B7BB8) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0x6B7BB8) }
                        }
                    }
                }
            }

            AppColors::Plum => {
                match app_theme {
                    AppThemes::Peach => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0xD9B8E6) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0xD9B8E6) }
                        }
                    }
                    AppThemes::Midnight => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0x8A5F99) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0x8A5F99) }
                        }
                    }
                }
            }

            AppColors::Powder => {
                match app_theme {
                    AppThemes::Peach => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0xD4E6F1) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0xD4E6F1) }
                        }
                    }
                    AppThemes::Midnight => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0x7BA7CC) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0x7BA7CC) }
                        }
                    }
                }
            }

            AppColors::Red => {
                match app_theme {
                    AppThemes::Peach => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0xFFB8C5) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0xFFB8C5) }
                        }
                    }
                    AppThemes::Midnight => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0xCC6B7C) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0xCC6B7C) }
                        }
                    }
                }
            }

            AppColors::Rose => {
                match app_theme {
                    AppThemes::Peach => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0xF1B8D4) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0xF1B8D4) }
                        }
                    }
                    AppThemes::Midnight => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0xB85F8A) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0xB85F8A) }
                        }
                    }
                }
            }

            AppColors::Sage => {
                match app_theme {
                    AppThemes::Peach => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0xC5E6D0) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0xC5E6D0) }
                        }
                    }
                    AppThemes::Midnight => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0x6B9B7C) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0x6B9B7C) }
                        }
                    }
                }
            }

            AppColors::Salmon => {
                match app_theme {
                    AppThemes::Peach => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0xFFCAB8) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0xFFCAB8) }
                        }
                    }
                    AppThemes::Midnight => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0xCC7F5C) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0xCC7F5C) }
                        }
                    }
                }
            }

            AppColors::Seafoam => {
                match app_theme {
                    AppThemes::Peach => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0xB8F1E6) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0xB8F1E6) }
                        }
                    }
                    AppThemes::Midnight => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0x5CB8A3) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0x5CB8A3) }
                        }
                    }
                }
            }

            AppColors::Sky => {
                match app_theme {
                    AppThemes::Peach => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0xB8D4F1) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0xB8D4F1) }
                        }
                    }
                    AppThemes::Midnight => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0x4A7BA7) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0x4A7BA7) }
                        }
                    }
                }
            }

            AppColors::Thistle => {
                match app_theme {
                    AppThemes::Peach => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0xE6C5F1) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0xE6C5F1) }
                        }
                    }
                    AppThemes::Midnight => {
                        match strength {
                            AppColorStrengths::Secondary => { AppColors::color_from_hex(0x9B6FB8) }
                            AppColorStrengths::Base => { AppColors::color_from_hex(0x9B6FB8) }
                        }
                    }
                }
            }
        }
    }
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

    /// Gets the theme's background color.
    fn background(&self) -> Color {
        AppColors::Background.themed(self, AppColorStrengths::Base)
    }

    /// Gets the theme's text color.
    fn text(&self) -> Color {
        AppColors::Text.themed(self, AppColorStrengths::Base)
    }

    /// Gets the theme's primary color.
    fn primary(&self) -> Color {
        AppColors::Accent.themed(self, AppColorStrengths::Base)
    }

    /// Gets the theme's success color.
    fn success(&self) -> Color {
        AppColors::Success.themed(self, AppColorStrengths::Base)
    }

    /// Gets the theme's warning color.
    fn warning(&self) -> Color {
        AppColors::Unavailable.themed(self, AppColorStrengths::Base)
    }

    /// Gets the theme's danger color.
    fn danger(&self) -> Color {
        AppColors::Danger.themed(self, AppColorStrengths::Base)
    }
}