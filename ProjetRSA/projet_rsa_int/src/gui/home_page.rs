// Modules ... 
use iced::theme::{Theme};
use iced::widget::{button, container, text,Container, Button, Column, Text};
use iced:: { Alignment, Sandbox, Settings, Element, Background, Shadow, Vector, Border, Padding,Length};
use iced::alignment::{Horizontal, Vertical};

use super::gui;
use super::components;


/// Structure représentant la page d'accueil.
///
/// Elle peut être étendue pour inclure des éléments interactifs supplémentaires.
pub struct HomePage {
    // Ajouter d’autres boutons si nécessaire
}

/// Structure représentant la page d'accueil.
///
/// Elle peut être étendue pour inclure des éléments interactifs supplémentaires.

impl HomePage {
    pub fn new() -> Self {Self {}}

    ///Affiche le contenu de la page d'accueil
    pub fn view(&self) -> Element<gui::Message> {
        Column::new()
            .push(text("Bienvenue sur notre application de test sur RSA!"))
            // Bouton pour naviguer vers la page de test de validité du chiffrement RSA
            .push(Button::new(Text::new("Validité RSA Chiffrement"))
                .on_press(gui::Message::Router(gui::Page::ValiditeRSAChiffrement)))
            //Bouton pour naviguer vers la page de test de sécurité du chiffrement RSA
            .push(Button::new(Text::new("Sécurité RSA Chiffrement"))
                .on_press(gui::Message::Router(gui::Page::ValiditeRSAChiffrement)))
            // Bouton pour naviguer vers la page de test de validité de la signature RSA
            .push(Button::new(Text::new("Validité RSA Signature"))
                .on_press(gui::Message::Router(gui::Page::ValiditeRSASignature)))
            // Bouton pour naviguer vers la page de test de sécurité de la signature RSA
            .push(Button::new(Text::new("Sécurité RSA Signature"))
                .on_press(gui::Message::Router(gui::Page::ValiditeRSAChiffrement)))
            // Ajouter les autres boutons ici
            .into()
    }


    /// Met à jour l'état de la page en fonction des messages reçus.
    ///
    /// # Arguments
    ///
    /// * `_message` - Le message spécifique à cette page.
    ///
    /// Remarque : Aucun comportement spécifique ici pour la page d'accueil.
    pub fn update(&mut self, _message: gui::PageMessage) {
        // Pas de logique spécifique ici pour la page d'accueil
    }
}

