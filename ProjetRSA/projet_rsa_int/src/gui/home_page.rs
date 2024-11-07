// Modules ... 
use iced::theme::{Theme};
use iced::widget::{button, container, text,Container, Button, Column, TextInput};
use iced:: { Alignment, Sandbox, Settings, Element, Background, Shadow, Vector, Border, Padding,Length};
use iced::alignment::{Horizontal, Vertical};

use super::gui;
use super::components;

pub fn page_accueil() -> Container<'static, gui::Message> {
    let column = Column::new()
       .push(text("Bienvenue sur notre application de test sur RSA!"));
    container(column)
        .padding(Padding::from(20))
        .style(iced::theme::Container::Custom(Box::new(components::ContainerStyle)))
}