#![allow(clippy::elidable_lifetime_names)]
// I follow the lifetime notation/elision suggestions in my editor (Zed).
#![windows_subsystem = "windows"]

use crate::container::app::App;

pub mod vault;
pub mod container;
pub mod pages;

fn main() -> iced::Result {
    // there have been some rendering issues on Fedora, and this fixed it
    #[cfg(target_os = "linux")]
    unsafe { std::env::set_var("WGPU_BACKEND", "gl"); }

    iced::application(App::new, App::update, App::view)
        .theme(App::theme)
        .subscription(App::subscription)
        .run()
}