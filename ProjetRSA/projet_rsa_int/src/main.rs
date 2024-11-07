mod rsa;
mod gui;
use iced::{Sandbox, Settings};
use gui::gui::RustUI;

fn main() -> iced::Result{
    RustUI::run(Settings::default())
}