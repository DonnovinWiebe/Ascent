#![allow(clippy::elidable_lifetime_names)]
// I follow the lifetime notation/elision suggestions in my editor (Zed).
#![windows_subsystem = "windows"]

pub mod application;
pub mod pages;
pub mod ui;

use crate::application::app::App;

fn main() -> iced::Result {
    // there have been some rendering issues on Fedora, and this fixed it
    #[cfg(target_os = "linux")]
    unsafe { std::env::set_var("WGPU_BACKEND", "gl"); }

    iced::application(App::new, App::update, App::view)
        .theme(App::iced_theme)
        .subscription(App::subscription)
        .run()
}