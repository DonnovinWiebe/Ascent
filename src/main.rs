use crate::container::app::App;

pub mod vault;
pub mod container;
pub mod ui;

fn main() -> iced::Result {
    iced::run(App::update, App::view)
}
