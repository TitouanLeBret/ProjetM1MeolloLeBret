pub mod rsa;
pub mod gui;
use iced::{Sandbox, Settings};
use gui::gui::App;


//Fonction main qui lance uniquement une instance de notre App, avec les settings par défaut
fn main() -> iced::Result{
    App::run(Settings::default())
}