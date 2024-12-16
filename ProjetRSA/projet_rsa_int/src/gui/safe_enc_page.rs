use iced::widget::{button, container, text, Button, Checkbox, Column, Row, Text};
use iced::{Alignment, Element, Length};
use iced::alignment::Horizontal;
use iced::Color;

use super::gui;
use crate::rsa::safe_enc;
use super::components::MyTextInput;
use crate::rsa::utils::TestStatus; // Pour utiliser directement TestStatus et pas check_enc::TestStatus

use crate::rsa::utils::all_status_to_false; // Pour remettre tous les status à false
use crate::rsa::safe_enc::ALL_TEST_STATUS_SECU_RSA; // Pour utiliser la liste de check_enc.rs


#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct SafeRsaChifPage {
    /// Valeur de N (clé publique).
    n_value: String,
    /// Valeur de e (exposant public).
    e_value: String,
    /// Valeur de ct (texte chiffré).
    ct_value: String,
    /// État du bouton pour vérifier la validité des clés.
    check_button: button::State,

    // Liste des messages d'erreur
    error_messages: Vec<String>,

    //Message déchiffré
    decrypted_message : String,
}

/// Crée une nouvelle instance de `ValidRsaChifPage` avec des valeurs initiales vides.
impl SafeRsaChifPage {
    pub fn new() -> Self {
        Self {
            n_value: String::new(),
            e_value: String::new(),
            ct_value: String::new(),
            check_button: button::State::new(),
            error_messages: Vec::new(),
            decrypted_message : String::new(),
        }
    }

    /// Met à jour les valeurs de clé publique et texte chiffré.
    ///
    /// # Arguments
    /// * `n_val` - Nouvelle valeur pour N.
    /// * `e_val` - Nouvelle valeur pour e.
    /// * `ct_val` - Nouvelle valeur pour ct.
    pub fn update(&mut self, n_val: String, e_val: String, ct_val: String) {
        self.n_value = n_val;
        self.e_value = e_val;
        self.ct_value = ct_val;
    }

    /// Génère la vue de la page en affichant les champs de saisie, les boutons, et les résultats des tests.
    pub fn view(&self) -> Element<gui::Message> {
        let title = Text::new("Sécurité Chiffrement RSA")
            .size(48)
            .horizontal_alignment(Horizontal::Center)
            .style(Color::from_rgb(0.2, 0.2, 0.6));

        // Bouton pour générer une nouvelle clé RSA valide
        let new_values_button = button(text("Générer une clé RSA valide"))
            .padding(10)
            .on_press(gui::Message::NewValuesRsaEncSecu);

        // Section pour la clé publique et le texte chiffré
        let key_section = Column::new()
            .spacing(10)
            .push(Text::new("Clé publique et texte chiffré :"))
            .push(Row::new() // N : text field, e : text field, ct : text field
                .spacing(15)
                .push(Text::new("N :"))
                .push(MyTextInput::new("N", &self.n_value).width(Length::Fill).on_input(|n_value| {
                    all_status_to_false(&mut ALL_TEST_STATUS_SECU_RSA.lock().unwrap());
                    gui::Message::FieldChangedRsaChiffSecu(n_value, self.e_value.clone(), self.ct_value.clone())
                }))
                .push(Text::new("e :"))
                .push(MyTextInput::new("e", &self.e_value).width(Length::Fill).on_input(|e_value| {
                    all_status_to_false(&mut ALL_TEST_STATUS_SECU_RSA.lock().unwrap());
                    gui::Message::FieldChangedRsaChiffSecu(self.n_value.clone(), e_value, self.ct_value.clone())
                }))
                .push(Text::new("ct :"))
                .push(MyTextInput::new("ct", &self.ct_value).width(Length::Fill).on_input(|ct_value| {
                    all_status_to_false(&mut ALL_TEST_STATUS_SECU_RSA.lock().unwrap());
                    gui::Message::FieldChangedRsaChiffSecu(self.n_value.clone(), self.e_value.clone(), ct_value)
                }))
            );

        // Bouton pour vérifier la validité des clés RSA
        let check_button = button(text("Vérifier la sécurité"))
            .padding(10)
            .on_press(gui::Message::CheckButtonPressedRsaChiffSecu);

        // Section des tests de sécurité avec les cases validées ou non
        let test_results = ALL_TEST_STATUS_SECU_RSA.lock().unwrap().iter()
            .fold(Column::new().spacing(10), |column: Column<'_, gui::Message>, test| {
                column.push(
                    Row::new()
                        .spacing(10)
                        .push(Text::new(test.name))
                        .push(Checkbox::new("", test.is_valid))
                )
            });

        // Section des messages d'erreur
        let error_message_section: Column<'_, gui::Message> = self.error_messages.iter().fold(
            Column::new().spacing(5),
            |column, error| {
                column.push(Text::new(error.clone()).style(Color::from_rgb(0.8, 0.2, 0.2)))
            },
        );

        //Section avec le message déchiffré
        let decrypted_message_section: Column<'_, gui::Message> = Column::new().spacing(5)
            .push(Text::new("Message déchiffré : "))
            .push(
                Text::new(self.decrypted_message.clone())
            );

        let wrapper = Column::new()
            .align_items(Alignment::Center)
            .spacing(20)
            .push(Button::new(Text::new("Retour a la page d'accueil"))
                .on_press(gui::Message::Router(gui::Page::Home)))
            .push(title)
            .push(new_values_button)
            .push(key_section)
            .push(check_button)
            .push(decrypted_message_section)
            .push(test_results)
            .push(error_message_section);

        container(wrapper)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_x()
            .center_y() // Fond standard
            .into()
    }

    /// Retourne l'état actuel des tests de sécurité.
    pub fn get_tests_status(&self) -> Vec<TestStatus> {
        ALL_TEST_STATUS_SECU_RSA.lock().unwrap().clone()
    }

    /// Ajoute un message d'erreur.
    pub fn add_error_message(&mut self, msg: &str) {
        self.error_messages.push(msg.to_string());
    }

    /// Supprime tous les messages d'erreur.
    pub fn remove_all_error_message(&mut self) {
        self.error_messages.clear();
    }

    pub fn display_message(&mut self, msg: &str){
        self.decrypted_message += &(msg.to_string()+&String::from("\n"));
    }

    pub fn remove_display_message(&mut self) {
        self.decrypted_message = String::new();
    }

    /// Vérifie la validité de la clé RSA en exécutant tous les tests de sécurité.
    pub fn check_values(&mut self) {
        self.remove_all_error_message();
        match validate_inputs(&self.n_value, &self.e_value, &self.ct_value) {
            Ok(_) => {
                safe_enc::calc_all_safety_status(self,
                    self.n_value.clone(),
                    self.e_value.clone(),
                    self.ct_value.clone(),
                );
            }
            Err(errors) => {
                let error_message = format!(
                    "Attention, les erreurs suivantes ont été détectées :\n{}",
                    errors.join("\n")
                );
                self.add_error_message(&error_message);
            }
        }
    }


    pub fn reset_status(&mut self){//Permet a gui d'appeler all_status_to_false, sans connaitre ALL_TEST_STATUS_SECU_RSA
        all_status_to_false(&mut ALL_TEST_STATUS_SECU_RSA.lock().unwrap());
    }

}

use std::str::FromStr;
/// Fonction de validation des entrées (vérifie que ce sont bien des entiers valides).
fn validate_inputs(n_value: &str, e_value: &str, ct_value: &str) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();

    // Essayer de convertir chaque valeur
    if num_bigint::BigUint::from_str(n_value).is_err() {
        errors.push("N n'est pas un entier valide.".to_string());
    }
    if num_bigint::BigUint::from_str(e_value).is_err() {
        errors.push("E n'est pas un entier valide.".to_string());
    }
    if num_bigint::BigUint::from_str(ct_value).is_err() {
        errors.push("CT n'est pas un entier valide.".to_string());
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
