mod rsa;
mod gui;
use iced::{Sandbox, Settings};
use gui::gui::App;

fn main() -> iced::Result{
    App::run(Settings::default())
}