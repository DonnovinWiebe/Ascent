use iced::system::theme;
use crate::container::app::App;

pub mod vault;
pub mod container;
pub mod ui;
mod pages;

fn main() -> iced::Result {
    unsafe { std::env::set_var("WGPU_BACKEND", "gl"); } // temporary debugging

    iced::application(App::new, App::update, App::view)
        .theme(App::theme)
        .run()
}