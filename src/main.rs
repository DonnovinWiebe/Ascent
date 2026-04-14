#![allow(clippy::elidable_lifetime_names)]
// I follow the lifetime notation/elision suggestions in my editor (Zed).

use crate::container::app::App;

pub mod vault;
pub mod container;
pub mod ui;
pub mod pages;

fn main() -> iced::Result {
    //unsafe { std::env::set_var("WGPU_BACKEND", "gl"); } // todo remove? re: temporary debugging for linux only?
    
    iced::application(App::new, App::update, App::view)
        .theme(App::theme)
        .run()
}