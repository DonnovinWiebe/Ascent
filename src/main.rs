use iced::system::theme;
use crate::container::app::App;

pub mod vault;
pub mod container;
pub mod ui;

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .theme(App::theme)
        .run()
}
