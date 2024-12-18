// Modules
use iced::widget::{text, Button, Column, Text};
use iced::Element;
use super::gui;



/// Structure représentant la page d'accueil.
pub struct HomePage {
    // Ajouter d’autres boutons si nécessaire
}



impl HomePage {
    /// Crée une nouvelle instance de la page d'accueil.
    /// À utiliser lors de l'initialisation de l'application.
    pub fn new() -> Self {Self {}}

    ///Affiche le contenu de la page d'accueil
    ///
    /// Cette méthode génère une colonne contenant :
    /// - Un texte de bienvenue
    /// - Deux boutons pour naviguer vers les pages de test (validité et sécurité du chiffrement RSA)
    ///

    pub fn view(&self) -> Element<gui::Message> {
        Column::new()
            .push(text("Bienvenue sur notre application de test sur RSA!"))
            // Bouton pour naviguer vers la page de test de validité du chiffrement RSA
            .push(Button::new(Text::new("Validité RSA Chiffrement"))
                .on_press(gui::Message::Router(gui::Page::ValiditeRSAChiffrement)))
            //Bouton pour naviguer vers la page de test de sécurité du chiffrement RSA
            .push(Button::new(Text::new("Sécurité RSA Chiffrement"))
                .on_press(gui::Message::Router(gui::Page::SecuriteRsaChiffrement)))
            // Ajouter les autres boutons ici
            .into()
    }
}

