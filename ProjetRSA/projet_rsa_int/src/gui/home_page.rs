// Modules ... 
use iced::theme::{Theme};
use iced::widget::{button, container, text,Container, Button, Column, Text};
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

pub struct HomePage {
    // Ajouter d’autres boutons si nécessaire
}

impl HomePage {
    pub fn new() -> Self {Self {}}
}

impl gui::PageContent for HomePage {
    fn view(&self) -> Element<gui::Message> {
        Column::new()
            .push(text("Bienvenue sur notre application de test sur RSA!"))
            .push(Button::new(Text::new("Validité RSA Chiffrement"))
                .on_press(gui::Message::Router(gui::Page::ValiditeRSAChiffrement)))
            .push(Button::new(Text::new("Sécurité RSA Chiffrement"))
                .on_press(gui::Message::Router(gui::Page::ValiditeRSAChiffrement)))
            .push(Button::new(Text::new("Validité RSA Signature"))
                .on_press(gui::Message::Router(gui::Page::ValiditeRSAChiffrement)))
            .push(Button::new(Text::new("Sécurité RSA Signature"))
                .on_press(gui::Message::Router(gui::Page::ValiditeRSAChiffrement)))
            // Ajouter les autres boutons ici
            .into()
    }

    fn update(&mut self, _message: gui::PageMessage) {
        // Pas de logique spécifique ici pour la page d'accueil
    }
}